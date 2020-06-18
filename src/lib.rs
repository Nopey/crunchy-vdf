use std::fmt::Debug;
pub mod parse;

#[derive(PartialEq)]
pub enum Item<'a> {
	String(&'a str),
	Integer(u32), // not u64 cos speed, nobodys testing for it anyways
	Many(Many<'a>),
}

impl<'a> Debug for Item<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Item::*;
		match self {
			String(s) => write!(f, "{:?}", s),
			Integer(u) => write!(f, "{:?}u32", u),
			Many(m) => std::fmt::Debug::fmt(m, f),
		}
	}
}

//TODO: maybe make this a type rather than a typedef.
pub type Pair<'a> = (&'a str, Item<'a>);

#[derive(PartialEq)]
pub struct Many<'a> (Box<[Pair<'a>]>);

impl<'a> Debug for Many<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_map()
			.entries(self.0.iter()
				.map(|&(ref k, ref v)| (k, v))
			).finish()
	}
}
