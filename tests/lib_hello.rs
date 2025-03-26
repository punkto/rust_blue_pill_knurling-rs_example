#![no_std]
#![no_main]

use app_with_tests as _; // global logger + panicking-behavior + memory layout

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use app_with_tests::lib_hello::call_hello;
    use defmt::assert;

    #[test]
    fn call_hello_returns_1_from_tests() {
        assert!(call_hello() == 1)
    }
}
