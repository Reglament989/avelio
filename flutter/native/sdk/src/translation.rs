use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use unic_langid::LanguageIdentifier;

pub struct Translations {
    pub bundle: FluentBundle<FluentResource>,
}

impl Default for Translations {
    fn default() -> Self {
        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed.");
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle
            .add_resource(fl_res!("name-of-the-company = Avelio"))
            .expect("Failed to add FTL resources to the bundle.");
        Self { bundle }
    }
}

impl Translations {
    pub fn message<K: AsRef<str>>(&self, key: K, args: FluentArgs) -> String {
        let mut errors = vec![];
        let msg = self
            .bundle
            .get_message(key.as_ref())
            .expect("Message doesn't exist.");
        let pattern = msg.value().expect("Message has no value.");
        self.bundle
            .format_pattern(&pattern, Some(&args), &mut errors)
            .to_string()
    }
}

#[repr(C)]
pub struct TranslateArg {
    pub name: *const c_char,
    pub value: *const c_char,
}

#[no_mangle]
pub extern "C" fn tr(
    key_translation: *const c_char,
    size_args: usize,
    array_pointer: *const c_char,
) -> *mut c_char {
    let mut normal_args = FluentArgs::new();
    if size_args > 0 {
        let args = unsafe {
            std::slice::from_raw_parts(array_pointer as *const TranslateArg, size_args as usize)
        };
        for arg in args {
            normal_args.set(string!(arg.name), string!(arg.value));
        }
    }
    CString::new(crate::FL.with(|tr| tr.message(string!(key_translation), normal_args)))
        .unwrap()
        .into_raw()
    // cstr!()
}
