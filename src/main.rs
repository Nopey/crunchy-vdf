use crunchy_vdf::parse::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
const TEST_VDF: &'static [u8] = include_bytes!("test.vdf");

#[no_mangle]
pub fn parse_test_vdf() -> crunchy_vdf::Many<'static> {
    let parsed = parse_vdf(TEST_VDF);
    let parsed = parsed.map_err(|e|
        e.map(|e| (std::str::from_utf8(e.0).unwrap(), e.1))
    );
    // println!("{:#?}", parsed.unwrap().1);
    parsed.unwrap().1
}
fn main() {
    for _ in 0..1000{
        parse_test_vdf();
    }
}
