use wasm_bindgen::prelude::*;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn compile(source_code: &str) -> Box<[u8]> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    /*#[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())*/
    
    Box::new([0,1,2,3])
}
