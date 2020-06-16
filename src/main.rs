use crunchy_vdf::parse;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let parsed = parse("\"it works on spen_testfile\" { \"and the\" \"maps\" } ");
    println!("Final: {:#?}", parsed);
}
