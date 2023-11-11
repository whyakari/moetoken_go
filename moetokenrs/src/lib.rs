use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use parking_lot::Mutex;
use tiktoken_rs::CoreBPE;
use tiktoken_rs::tokenizer::{Tokenizer, get_tokenizer};

#[no_mangle]
pub extern "C" fn ohayou_oniichan(name: *const c_char) -> *mut c_char {
    let name: &str = unsafe { CStr::from_ptr(name).to_str().unwrap() };

    let result: String = format!("Ohayou, {}!", name);
    let result: CString = CString::new(result).unwrap();

    result.into_raw()
}

pub fn get_token(t: Tokenizer) -> Result<Arc<Mutex<CoreBPE>>> {
    let token_ = match t {
        Tokenizer::Cl100kBase => tiktoken_rs::cl100k_base_singleton(),
        Tokenizer::P50kBase => tiktoken_rs::p50k_base_singleton(),
        Tokenizer::R50kBase => tiktoken_rs::r50k_base_singleton(),
        Tokenizer::P50kEdit => tiktoken_rs::p50k_edit_singleton(),
        Tokenizer::Gpt2 => tiktoken_rs::r50k_base_singleton(),
    };
    Ok(token_)
}

#[no_mangle]
pub extern "C" fn get_qtd_tokens(model_name: *const libc::c_char, txt: *const libc::c_char) -> libc::c_uint {
    let model_name: &str = unsafe {
        CStr::from_ptr(model_name).to_str().unwrap()
    };

    let txt: &str = unsafe {
        CStr::from_ptr(txt).to_str().unwrap()
    };

    let tokenizer = get_tokenizer(model_name).ok_or_else( ||anyhow!("Model {} does not exist!", model_name)).unwrap();

    let moe = get_token(tokenizer).unwrap();
    let result: usize = moe.lock().encode_with_special_tokens(txt).len();

    result as libc::c_uint
}

mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_get_qtd_token_uwu() {
        let model_name = CString::new("gpt-3.5-turbo").unwrap();
        let txt = CString::new("Ohayou, Oniichan!").unwrap();
        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 9);

    }

    #[test]
    fn test_get_qtd_token_uwu_2() {
        let model_name = CString::new("nothing").unwrap();
        let txt = CString::new("Ohayou, Oniichan!").unwrap();
        assert_eq!(get_qtd_tokens(model_name.as_ptr(), txt.as_ptr()), 0);
    }
}
