pub mod sampler;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    const N_STEP: usize = 100;
    const N_DIM: usize = 2;
    let mut state = sampler::State::<N_DIM>::new(0xdeadbeef);
    for _ in 0..N_STEP {
        state.take_step();
        log(&format!("{:?}, {:?}", state.arr[0], state.arr[1]));
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello from wasm");
}