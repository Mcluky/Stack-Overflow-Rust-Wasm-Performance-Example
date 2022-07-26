mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rustperformance!");
}

#[wasm_bindgen]
pub fn bench_rs(max: u64) -> u64 {
    (1..=max).map(|n| calculate_is_prime_rs(n) as u64).sum()
}


pub fn calculate_is_prime_rs(number: u64) -> bool {
    if number == 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
