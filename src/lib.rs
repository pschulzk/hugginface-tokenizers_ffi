use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use tokenizers::Tokenizer;
use once_cell::sync::Lazy;

// Embed the tokenizer JSON as a byte array
static TOKENIZER_JSON: &[u8] = include_bytes!("../../assets/tokenizer.json");

// Use once_cell to lazily initialize the tokenizer
static TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| {
    Tokenizer::from_bytes(TOKENIZER_JSON).expect("Failed to load tokenizer")
});

#[no_mangle]
pub extern "C" fn tokenize(input: *const c_char) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let input_str = unsafe { CStr::from_ptr(input).to_str().unwrap() };

        println!("Input: {}", input_str);

        let encoding = TOKENIZER.encode(input_str, false).unwrap();
        let ids = encoding.get_ids();
        
        println!("Number of tokens: {}", ids.len());

        let result = ids.iter()
            .map(|&id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("Result string length: {}", result.len());

        CString::new(result).unwrap().into_raw()
    });

    match result {
        Ok(ptr) => ptr,
        Err(_) => CString::new("Error during tokenization").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            drop(CString::from_raw(s));
        }
    };
}