include!(concat!(env!("OUT_DIR"), "/test_mod.rs"));

fn main() {
    test_mod::test();
}
