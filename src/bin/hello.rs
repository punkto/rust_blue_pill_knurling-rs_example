#![no_main]
#![no_std]

use app_with_tests::{self as _, lib_hello::call_hello}; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    call_hello();

    app_with_tests::exit()
}


