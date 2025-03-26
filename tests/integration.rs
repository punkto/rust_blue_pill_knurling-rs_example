#![no_std]
#![no_main]

use app_with_tests as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn test_at_integration_test() {
        assert!(true)
    }
}
