use crunchy_vdf::parse::*;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
const TEST_VDF: &'static [u8] = include_bytes!("test.vdf");

#[no_mangle]
pub fn parse_test_vdf() -> crunchy_vdf::Many<'static> {
    let parsed = parse_vdf(TEST_VDF).unwrap().1;

    #[cfg(debug_assertions)]
    println!("{:#?}", parsed);

    parsed
}

fn main() {
    #[cfg(parallel)]
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    #[cfg(debug_assertions)]
    let max = 1;
    #[cfg(not(debug_assertions))]
    let max = 1000;
    for _ in 0..max{
        parse_test_vdf();
    }
}
