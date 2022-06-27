use std::{path::PathBuf, fs};

use v8::{Global, Module, TryCatch, HandleScope};

mod json;
mod js;
mod resource;
pub use js::AvModJS;
pub use json::AvModJSON;
pub use resource::{Resource, SourceFile};

pub struct AvMod {

}

impl AvMod {
    pub fn load_file(path: &PathBuf) -> String {
        fs::read_to_string(path).unwrap()
    }

    pub fn load_from_file<'a>(scope: &mut HandleScope<'a>, dir: &PathBuf, file: SourceFile) -> Result<Global<Module>, String> {
        let p = file.to_path(dir.to_str().unwrap().to_string())?;
        match file {
            f if f.is_js_file() => AvModJS::load_module(&mut TryCatch::new(scope), &p),
            f if f.extension() == "json" => AvModJSON::load_module(&mut TryCatch::new(scope), &p),
            f => Err(format!("Unrecognised file format {}", f.extension()))
        }
    }
}

pub trait AvModProvider {
    /// Loads a module from a (verified to exist) path.
    /// ### Parameters:
    /// * scope -- Try-Catch V8 Scope
    /// * path -- path to file
    fn load_module<'a>(
        scope: &mut TryCatch<HandleScope<'a>>,
        path : &PathBuf
    ) -> Result<Global<Module>, String>;

    fn _instantiate_callback<'a>(
        context: v8::Local<'a, v8::Context>,
        specifier: v8::Local<'a, v8::String>,
        import_assertions: v8::Local<'a, v8::FixedArray>,
        referrer: v8::Local<'a, v8::Module>,
    ) -> Option<v8::Local<'a, v8::Module>>;
}