use std::collections::HashMap;

use v8::{HandleScope, Local, Object,};

// mod clipboard;
pub mod debug;
mod shell;
mod pipe;
mod net;

use crate::core::JSApi;

// use clipboard::AvClipboard;
use shell::AvShell;
use self::{debug::AvDebug, net::AvNet};

pub struct AvdanAPI {}

impl AvdanAPI {

    fn assign_mod_to_obj<'a>(scope: &mut HandleScope<'a>, obj: Local<Object>, name: &str, module: &impl JSApi) -> () {
        let k = v8::String::new(scope, name).unwrap();
        let tmp = module.js(scope);
        obj.define_own_property(scope, k.into(), tmp.into(), v8::READ_ONLY);
    }
    

    fn assign_values<'a>(scope: &mut HandleScope<'a>, obj : Local<Object>) -> () {
        // Self::assign_mod_to_obj(scope, obj, "Clipboard", &AvClipboard {});
        Self::assign_mod_to_obj(scope, obj, "Debug", &AvDebug {});
        Self::assign_mod_to_obj(scope, obj, "Shell", &AvShell {});
    }

    pub fn public_apis<'a>() -> HashMap<&'static str, Box<dyn JSApi>> {
        let mut h : HashMap<_, Box<dyn JSApi>> = HashMap::new();
        // h.insert("clipboard", Box::new(AvClipboard{}));
        h.insert("debug", Box::new(AvDebug {}));
        h.insert("shell", Box::new(AvShell {}));
        h.insert("net", Box::new(AvNet {}));
        h
    }
}

impl JSApi for AvdanAPI {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        let obj = v8::Object::new(scope);
        Self::assign_values(scope, obj);
        obj
    }
}