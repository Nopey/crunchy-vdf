use std::fmt::Debug;
pub mod parse;

#[derive(PartialEq)]
pub enum Item {
	String(String),
	Integer(u32), // not u64 cos speed, nobodys testing for it anyways
	Many(Many),
}

impl Debug for Item {
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
pub type Pair = (String, Item);

#[derive(PartialEq)]
pub struct Many (Box<[Pair]>);

impl Debug for Many {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_map()
			.entries(self.0.iter()
				.map(|&(ref k, ref v)| (k, v))
			).finish()
	}
}
