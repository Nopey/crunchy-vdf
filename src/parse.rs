use crate::*;
use nom::IResult;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::branch::*;

//TODO: API: document functions, even private ones. (not tests tho)
//TODO: test: test more error conditions, like newline on empty string
//TODO: perhaps switch from [u8] to char

/// Parse a Valve Keyvalues file
pub fn parse_vdf<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], Many> {
    all_consuming(parse_many)(string)
}

/// Parses a keyvalues pair
fn parse_pair<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], Option<Pair>> {
    // Parse the key, either quoted or unquoted string.
    let (string, key) = parse_auto_string(string)?;

    // whitespace
    let (string, _) = tuple((whitespace, opt(tuple((
        tag("="),
        whitespace
    )))))(string)?;

    // let (string, c) = take(1usize)(string)?;
    // let c = *unsafe{c.get_unchecked(0)};
    let c = string.get(0);

    if c==Some(&b'[') {
        // if next char is '[', parse conditional and then sub_pair
        let (string, cond) = parse_conditional(string)?;
        if cond {
            let (string, value) = parse_many_with_braces(string)?;
            Ok((string, Some((key, Item::Many(value)))))
        }else {
            let (string, ()) = skip_braces(string)?;
            Ok((string, None))
        }
    } else if c==Some(&b'{') || c==Some(&b'\n') || c==Some(&b'\r') {
        // if next char is '{', parse sub_pair
        let (string, ()) = newline_maybe(string)?;
        let (string, value) = parse_many_with_braces(string)?;
        Ok((string, Some((key, Item::Many(value)))))
    } else {
        // else, parse simple pair and maybe conditional.
        let (string, value) = parse_auto_string(string)?;
        Ok((string, Some((key, Item::String(value)))))
    }
}

#[test]
fn test_parse_pair() {
    todo!()
}

fn skip_braces_inner<'a> ( mut string: &'a [u8] ) -> IResult<&'a [u8], ()> {
    let mut depth = 0usize;
    let not_brace = |ref c| !b"{}".contains(c);
    loop {
        string = take_while(not_brace)(string)?.0;
        match string{
            [b'{', rest@..] => {
                depth += 1;
                string = rest;
            },
            [b'}', rest@..] => {
                depth = match depth.checked_sub(1) {
                    Some(e) => e,
                    None => return Ok((string, ()))
                };
                string = rest;
            },
            [] => return Ok((b"", ())),
            _ => unreachable!()
        }
    }
}

#[test]
fn test_skip_braces_inner() {
    // TODO: SLOW: Perhaps make this function consume the final }
    assert_eq!(
        skip_braces_inner(b"really {basic} }"),
        Ok((&b"}"[..], ()))
    );
    assert_eq!(
        skip_braces_inner(b"{{trailing ones are optional"),
        Ok((&b""[..], ()))
    );
    assert_eq!(
        skip_braces_inner(b"{the} } trailing content is preserved"),
        Ok((&b"} trailing content is preserved"[..], ()))
    );
}

/// Skips from an opening '{' to the closing '}'
/// its like parse_many_with_braces, but without parsing the many.
fn skip_braces<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], ()> {
    delimited(
        tuple((newline_maybe, tag("{"), newline_maybe)),
        skip_braces_inner,
        alt((tag("}"), all_consuming(rest)))
        // Alternatively, if EOF trailing braces are mandatory:
        // tag("}")
    )(string).map(|a| (a.0, ()))
}

#[test]
fn test_skip_braces() {
    todo!()
}

/// Wrapper around parse_many that adds braces and newline management
/// (sub_pair in pest grammar)
fn parse_many_with_braces<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], Many> {
    delimited(
        tuple((newline_maybe, tag("{"), newline_maybe)),
        parse_many,
        alt((tag("}"), all_consuming(rest)))
        // Alternatively, if EOF trailing braces are mandatory:
        // tag("}")
    )(string)
}

#[test]
fn test_parse_many_with_braces() {
    todo!()
}

fn parse_many<'a> ( mut string: &'a [u8] ) -> IResult<&'a [u8], Many> {
    let mut many = vec![];
    loop {
        // mandatory newline? I don't feel like it, and apparently neither did valve.
        // my PEST grammar did that, but it's a new day.

        string = newline_maybe(string)?.0;

        // If we've hit EOF or end of sub_pair
        if match string.get(0) {
            Some(b'}') => true,
            None => true,
            _ => false
        } {
            // then don't parse a pair.
            break;
        }

        // parse pair, push result to Vec
        string = match parse_pair(string)? {
            (rest, Some(goodie)) => {
                many.push(goodie);
                rest
            },
            (rest, None) => rest,
        };
    }
    Ok((string, Many(many.into_boxed_slice())))
}
#[test]
fn test_parse_many() {
    todo!()
}

fn parse_conditional<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], bool> {
    // out of what? That's June for you..
    let not_out = |ref c| !b"]".contains(c);
    let (rest, _run) = delimited(tag("["), take_while(not_out), tag("]"))(string)?;
    // eprintln!("TODO: Conditionals! {:?}", unsafe{std::str::from_utf8_unchecked(run)});
    Ok((rest, true))
}

#[test]
fn test_parse_conditional(){
    // Tests the conditionals, as if m_bEvaluateConditionals==false
    assert_eq!(
        parse_conditional(br#"[$WIN32] bottom_text"#),
        Ok((&b" bottom_text"[..], true))
    );
}

#[test]
fn test_parse_conditional_and_evaluate(){
    todo!();
}

/// Parse either a quoted on unquoted string, automatically.
fn parse_auto_string<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], String> {
    alt((parse_string, parse_unquoted_string))(string)
}

#[test]
fn test_parse_auto_string() {
    assert_eq!(
        parse_auto_string(br#"" fun" rest of content"#),
        Ok((&b" rest of content"[..], " fun".to_owned()))
    );
    assert_eq!(
        parse_auto_string(br#""Escape Sequence \" ""#),
        Ok((&b""[..], "Escape Sequence \" ".to_owned()))
    );
    assert_eq!(
        parse_auto_string(br#"fun/path\to\my_asset.vtf bottom_text"#),
        Ok((&b" bottom_text"[..], "fun/path\\to\\my_asset.vtf".to_owned()))
    );
}

fn parse_unquoted_string<'a> ( mut string: &'a [u8] ) -> IResult<&'a [u8], String> {
    // I only hope this not_bad conditional isn't too slow.
    let not_bad = |ref c| !b"\"{}[]\t \n\r".contains(c);

    let (rest, run) = take_while1(not_bad)(string)?;
    string = rest;
    // Safe Version:
    // Ok((string, std::str::from_utf8_unchecked(run).unwrap().to_owned()))
    Ok((string, unsafe{std::str::from_utf8_unchecked(run)}.to_owned()))
}

#[test]
fn test_parse_unquoted_string(){
    assert_eq!(
        parse_unquoted_string(br#"fun/path\to\my_asset.vtf bottom_text"#),
        Ok((&b" bottom_text"[..], "fun/path\\to\\my_asset.vtf".to_owned()))
    );
}

fn escape_map( c: u8 ) -> char {
    match c{
        // b'0' => '\0', // hey, no.
        b'n' => '\n',
        b'r' => '\r',
        b't' => '\t',
        // one of the tests uses \d, but I don't know what that means.
        // so this code just turns \d into `d`.
        c => c.into(),
    }
}

// TODO: consider using https://docs.rs/nom/5.1.2/nom/bytes/complete/fn.escaped.html
fn parse_string_inner_escaped<'a> ( mut string: &'a [u8] ) -> IResult<&'a [u8], String> {
    // I should hope the files don't contain bs.. ;)
    // i also made it stop on quote marks, because that's the terminator we're looking for.
    let not_bs = |ref c| !b"\\\"".contains(c);
    let mut result = String::new();

    loop{
        let (rest, run) = take_while(not_bs)(string)?;
        string = rest;
        // Safe Version:
        // result.push_str(std::str::from_utf8(run).unwrap());
        result.push_str(unsafe{std::str::from_utf8_unchecked(run)});
        if let [b'\\', c, rest@.. ] = string
        {
            // TODO: SLOW: I'm really tempted to unsafely modify the input,
            // to avoid an allocation per string (extremely unsafely)
            // but I won't do that until I'm really squeezing for optimizations
            // (we'd shift the array somewhere in here, lots of unsafe magic :3)
            result.push(escape_map(*c));
            string = rest;
        }else{
            break;
        }
    }
    
    Ok((string, result))
}

fn parse_string<'a> ( string: &'a [u8] ) -> IResult<&'a [u8], String> {
    //TODO: unescaped version of inner, prob using parse_unquoted_string
    let inner = parse_string_inner_escaped;
    delimited(tag("\""), inner, tag("\""))(string)
}

#[test]
fn test_parse_string(){
    assert_eq!(
        parse_string(br#"" fun" rest of content"#),
        Ok((&b" rest of content"[..], " fun".to_owned()))
    );
    assert_eq!(
        parse_string(br#""Escape Sequence \" ""#),
        Ok((&b""[..], "Escape Sequence \" ".to_owned()))
    );
}

// Skips whitespace and inline comments (`/* ... */`)
fn whitespace<'a> (mut string: &'a [u8]) -> IResult<&'a [u8], ()>{
    let ws =  nom::character::is_space;

    loop{
        string = take_while(ws)(string)?.0;
        if tag::<_,_, (&[u8], nom::error::ErrorKind)>(b"/*")(string)
            .map_or(true, |t| t.1.is_empty())
        {
            break;
        }else{
            // JUST: can't win competitions with bounds checking in hot functions
            string = unsafe { take_until("*/")(string)?.0.get_unchecked(2..) };
            // Safe Version:
            // string = &take_until("*/")(string)?.0[2..];
        }
    }
    
    Ok((string, ()))
}

#[test]
fn test_whitespace(){
    assert_eq!(whitespace(b" "), Ok((&b""[..], ())));
    assert_eq!(whitespace(b" /* Comments work */ "), Ok((&b""[..], ())));
    assert_eq!(whitespace(b" /* Tab support */\t "), Ok((&b""[..], ())));
    assert_eq!(whitespace(b"I'm not a space!\n"), Ok((&b"I'm not a space!\n"[..], ())));
    assert_eq!(whitespace(b""), Ok((&b""[..], ())));
}

/// Skips whitespace, newlines and all comments (`// ...` or `/* */`)
/// NEW! Fails if no newline is consumed :3
fn newline<'a> (mut string: &'a [u8]) -> IResult<&'a [u8], ()>{
    let nl = |ref c| b"\n\r".contains(c);

    loop{
        //TODO: maybe be faster about it..
        string = whitespace(string)?.0;

        string = take_while1(nl)(string)?.0;
        string = whitespace(string)?.0;
        if tag::<_,_, (&[u8], nom::error::ErrorKind)>(b"//")(string).is_ok(){
            string = take_till(nl)(string)?.0;
        }else{
            break;
        }
    }

    Ok((string, ()))
}

#[test]
fn test_newline(){
    //TODO: Test the mandatory newlineness
    assert_eq!(newline(b"\n\r"), Ok((&b""[..], ())));
    assert_eq!(newline(b"\n//comments are eaten, too\n"), Ok((&b""[..], ())));
    assert_eq!(newline(b"\nI'm not a newline!"), Ok((&b"I'm not a newline!"[..], ())));
    assert_eq!(newline(b"\n // whitespace should get eaten\n"), Ok((&b""[..], ())));
    assert_eq!(whitespace(b" /* inline comments work */ "), Ok((&b""[..], ())));
}


/// Skips whitespace, newlines and all comments (`// ...` or `/* */`)
/// doesn't fail
fn newline_maybe<'a> (mut string: &'a [u8]) -> IResult<&'a [u8], ()>{
    let nl = |ref c| b"\n\r".contains(c);

    loop{
        //TODO: be faster about it..
        string = whitespace(string)?.0;

        string = take_while(nl)(string)?.0;
        string = whitespace(string)?.0;
        if tag::<_,_, (&[u8], nom::error::ErrorKind)>(b"//")(string).is_ok(){
            string = take_till(nl)(string)?.0;
        }else{
            break;
        }
    }

    Ok((string, ()))
}

#[test]
fn test_newline_maybe(){
    //TODO: Test the lack of mandatory newlineness
    assert_eq!(newline_maybe(b"\n\r"), Ok((&b""[..], ())));
    assert_eq!(newline_maybe(b"\n//comments are eaten, too\n"), Ok((&b""[..], ())));
    assert_eq!(newline_maybe(b"\nI'm not a newline!"), Ok((&b"I'm not a newline!"[..], ())));
    assert_eq!(newline_maybe(b"\n // whitespace should get eaten\n"), Ok((&b""[..], ())));
    assert_eq!(whitespace(b" /* inline comments work */ "), Ok((&b""[..], ())));
}
