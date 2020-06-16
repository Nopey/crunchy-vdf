use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, PartialEq)]
pub enum Item<'a> {
	String(&'a str),
	Integer(u32), // not u64 cos speed, nobodys testing for it anyways
	Many(Many<'a>),
}

impl<'a> Item<'a> {
	fn from_pair( pair: pest::iterators::Pair<'a, Rule> ) -> Self {
		match pair.as_rule(){
			Rule::simple_pair => {
				let value = pair.into_inner().next().unwrap().as_str();
				Item::String(value)
			},
			Rule::sub_pair => {
				let value = Many::from_pairs(pair.into_inner());
				Item::Many(value)
			},
			_ => panic!("unexpected pair: {:#?}", pair)
		}
	}
}

pub type Pair<'a> = (&'a str, Item<'a>);

#[derive(Debug, PartialEq)]
pub struct Many<'a> (Box<[Pair<'a>]>);
impl<'a> Many<'a> {
	fn from_pairs(pairs: pest::iterators::Pairs<'a, Rule>) -> Self {
		let mut map = Vec::new();
		// println!("{:#?}", pair);
		for pair in pairs {
			assert_eq!(pair.as_rule(), Rule::pair);
			let mut pairs = pair.into_inner();
			let key = pairs.next().unwrap();
			//assert_eq!(key.as_rule(), Rule::key);
			//let key = key.into_inner().next().unwrap()
			let key = key.as_str();
			let next = pairs.next().unwrap();
			if let Some(next) = match next.as_rule() {
				Rule::cond_expr => if evaluate_cond_expr(next.into_inner()) { pairs.next() }else { None },
				Rule::sub_pair => Some(next),
				Rule::simple_pair => Some(next),
				_ => unreachable!()
			}{
				map.push((key, Item::from_pair(next)));
			}
		}
		Many(map.into_boxed_slice())
	}
}

fn evaluate_cond_expr<'a> (pairs: pest::iterators::Pairs<'a, Rule>) -> bool {
	false
}

#[derive(Parser)]
#[grammar = "vdf.pest"]
struct KeyValuesParser;

pub fn parse<'a> ( string: &'a str ) -> Many<'a> {
	// dancing
	let pairs = KeyValuesParser::parse(Rule::file, string).unwrap_or_else(|e| panic!("{}", e));
	Many::from_pairs(pairs)
}