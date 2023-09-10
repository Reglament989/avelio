use prost::Message;

macro_rules! string {
    ($ptr:expr) => {
        string!($ptr, 0)
    };
    ($ptr:expr, $error:expr) => {{
        assert!(!$ptr.is_null());
        unsafe { CStr::from_ptr($ptr) }.to_str().unwrap().to_owned()
    }};
}

macro_rules! cstr {
    ($ptr:expr) => {
        cstr!($ptr, 0)
    };
    ($ptr:expr, $error:expr) => {{
        CString::new($ptr).unwrap().into_raw()
    }};
}

macro_rules! buf_to_vec {
    ($ptr:expr) => {
        buf_to_vec!($ptr, 0)
    };
    ($ptr:expr, $error:expr) => {{
        unsafe { std::slice::from_raw_parts_mut($ptr.data, $ptr.len) }.to_vec()
    }};
}

pub(crate) fn proto_to_vec<T>(proto: T) -> Vec<u8>
where
    T: Message,
{
    let mut buf = Vec::new();
    buf.reserve(proto.encoded_len());
    proto.encode(&mut buf).unwrap();
    buf
}
macro_rules! fl_res {
    ($ptr:expr) => {
        fl_res!($ptr, 0)
    };
    ($ptr:expr, $error:expr) => {{
        let ftl_string = $ptr.to_owned();
        FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.")
    }};
}
