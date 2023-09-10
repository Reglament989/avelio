use prost::bytes::Bytes;
use prost::Message;
use sdk::proto::general::Song;
use sdk::*;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::io::Cursor;
use std::os::raw::c_char;
use tokio::runtime::Runtime;
use translation::Translations;
use types::Buffer;
#[macro_use]
mod utils;

pub mod proto;
mod translation;
pub mod types;

use crate::utils::proto_to_vec;

thread_local! {
    pub static API: RefCell<Avelio> = RefCell::new(Avelio::default());
    pub static RT: Runtime = Runtime::new().unwrap();
    pub static FL: Translations = Translations::default();
}

#[no_mangle]
pub extern "C" fn init(base_url_ptr: *const c_char, token_ptr: *const c_char) {
    let base_url = string!(base_url_ptr);

    let token = string!(token_ptr);
    API.with(|api| api.replace(Avelio::new(base_url, Some(token))));
}

#[no_mangle]
pub extern "C" fn sign_in(login_ptr: *const c_char, password_ptr: *const c_char) -> *const c_char {
    let login = string!(login_ptr);
    let password = string!(password_ptr);

    API.with(|api| {
        let mut api = api.borrow_mut();
        let response = RT.with(|r| r.block_on(async { api.sign_in(login, password).await }));

        match response {
            Ok(tokens) => {
                api.token = tokens.token.clone();
                let v = proto_to_vec(tokens);
                cstr!(base64::encode(v))
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn sign_up(login_ptr: *const c_char, password_ptr: *const c_char) -> *const c_char {
    let login = string!(login_ptr);
    let password = string!(password_ptr);

    API.with(|api| {
        let mut api = api.borrow_mut();
        let response = RT.with(|r| r.block_on(async { api.sign_up(login, password).await }));
        match response {
            Ok(tokens) => {
                api.token = tokens.token.clone();
                let v = proto_to_vec(tokens);
                cstr!(base64::encode(v))
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn refresh_token(token_ptr: *const c_char) -> *const c_char {
    let token = string!(token_ptr);

    API.with(|api| {
        let mut api = api.borrow_mut();
        let response = RT.with(|r| r.block_on(async { api.refresh_token(token).await }));
        match response {
            Ok(tokens) => {
                api.token = tokens.token.clone();
                let v = proto_to_vec(tokens);
                cstr!(base64::encode(v))
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn tracks(limit: i64, offset: i64) -> *const c_char {
    API.with(|api| {
        let api = api.borrow();
        let response =
            RT.with(|r| r.block_on(async { api.tracks(Some(limit), Some(offset)).await }));
        match response {
            Ok(tracks) => {
                let v = proto_to_vec(tracks);
                cstr!(base64::encode(v))
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn track_by_id(id_ptr: *const c_char) -> *const c_char {
    API.with(|api| {
        let api = api.borrow();
        let id = string!(id_ptr);
        let response = RT.with(|r| r.block_on(async { api.track_by_id(id).await }));
        match response {
            Ok(song) => {
                let v = proto_to_vec(song);
                cstr!(base64::encode(v))
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn get_bytes_of_track(id_ptr: *const c_char) -> Buffer {
    API.with(|api| {
        let api = api.borrow();
        let id = string!(id_ptr);
        let response = RT.with(|r| r.block_on(async { api.get_bytes_of_track(id).await }));
        match response {
            Ok(bytes) => {
                let data = bytes.to_vec().as_mut_ptr();
                let len = bytes.len();
                let buf = Buffer { data, len };
                std::mem::forget(&buf);
                buf
            }
            Err(err) => {
                println!("{:#?}", err);
                let buf = Buffer {
                    data: 0 as *mut u8,
                    len: 0,
                };
                std::mem::forget(&buf);
                buf
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn upload_track(bytes_buf: Buffer, song_ptr: *const c_char) -> *const c_char {
    API.with(|api| {
        let api = api.borrow();
        let song = Song::decode(Cursor::new(base64::decode(string!(song_ptr)).unwrap())).unwrap();
        let bytes = Bytes::from(buf_to_vec!(bytes_buf));
        let response = RT.with(|r| r.block_on(async { api.upload_track(bytes, song).await }));
        match response {
            Ok(id) => {
                cstr!(id)
            }
            Err(err) => {
                println!("{:#?}", err);
                cstr!("")
            }
        }
    })
}
