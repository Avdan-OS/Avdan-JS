use std::{path::PathBuf, fs};

use colored::Colorize;
use v8::{Global, Module, TryCatch, HandleScope, Context, Local, Value, CallbackScope, PromiseResolver};

mod json;
mod js;
mod resource;
mod store;
mod internal;
pub use internal::AvModInternal;
pub(crate) use store::AvModStore;
pub use js::AvModJS;
pub use json::AvModJSON;
pub use resource::{Specifier, SourceFile};

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

    pub fn load<'a>(scope: &mut HandleScope<'a>, dir: &PathBuf, resource: Specifier) -> Result<Global<Module>, String> {
        match resource {
            Specifier::File(f) => Self::load_from_file(scope, dir, f),
            Specifier::Module(id) => todo!(),
            Specifier::Internal(id) => AvModInternal::get_internal_module(scope, id)
        }
    }

    fn instantiate_callback<'a>(
        _context: v8::Local<'a, v8::Context>,
        _specifier: v8::Local<'a, v8::String>,
        _import_assertions: v8::Local<'a, v8::FixedArray>,
        referrer: v8::Local<'a, v8::Module>,
     ) -> Option<v8::Local<'a, v8::Module>> {
        Some(referrer)
    }

    fn evaluation_steps<'a>(
        ctx: Local<'a, Context>,
        module: Local<'a, Module>
    ) -> Option<Local<'a, Value>> {
        let scope = &mut unsafe {
            CallbackScope::new(ctx)
        };
        let scope = &mut TryCatch::new(scope);

        let default_key = v8::String::new(scope, "default").unwrap();
        let r = module.set_synthetic_module_export(scope, 
            default_key.into(), default_key.into());

        match r {
            None => {
                let excep = scope.exception().unwrap();
                panic!("{}", format!("Error from JS:\n\t{}", excep.to_rust_string_lossy(scope).as_str().bright_red()));
            },
            Some(_) => {},
        }

        let out = v8::String::new(scope, "Module Eval Output").unwrap();

        let prom = PromiseResolver::new(scope).unwrap();
        let udef = v8::undefined(scope);

        prom.resolve(scope, udef.into());

        Some(prom.get_promise(scope).into())
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