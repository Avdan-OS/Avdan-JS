use v8::{HandleScope, Local, Object,};

mod clipboard;
mod file;
pub mod debug;
mod shell;
mod pipe;
mod net;

use crate::core::JSApi;

use clipboard::AvClipboard;
use file::AvFile;
use shell::AvShell;
use self::debug::AvDebug;

pub struct AvdanAPI {}

impl AvdanAPI {

    fn assign_mod_to_obj<'a>(scope: &mut HandleScope<'a>, obj: Local<Object>, name: &str, module: &impl JSApi) -> () {
        let k = v8::String::new(scope, name).unwrap();
        let tmp = module.js(scope);
        obj.define_own_property(scope, k.into(), tmp.into(), v8::READ_ONLY);
    }
    

    fn assign_values<'a>(scope: &mut HandleScope<'a>, obj : Local<Object>) -> () {
        Self::assign_mod_to_obj(scope, obj, "Clipboard", &AvClipboard {});
        Self::assign_mod_to_obj(scope, obj, "File", &AvFile {});
        Self::assign_mod_to_obj(scope, obj, "Debug", &AvDebug {});
        Self::assign_mod_to_obj(scope, obj, "Shell", &AvShell {});
    }
}

impl JSApi for AvdanAPI {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        let obj = v8::Object::new(scope);
        Self::assign_values(scope, obj);
        obj
    }
}