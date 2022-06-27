use std::intrinsics::transmute;

use v8::{Exception, Local, Value, Global, Module, Context, HandleScope, CallbackScope, TryCatch};

use super::{AvModProvider, AvMod};

pub struct AvModJSON {}

impl AvModJSON {
    fn eval_callback<'a>(context : Local<'a, Context>, module : Local<'a, Module>) -> Option<Local<'a, Value>> {
        let scope = &mut unsafe {
            CallbackScope::new(context)
        };
        let scope = &mut TryCatch::new(scope);

        println!("Importing a JSON module!");
        return None;
    }
}

impl AvModProvider for AvModJSON {
    fn load_module<'a>(
        scope: &mut v8::TryCatch<v8::HandleScope<'a>>,
        path : &std::path::PathBuf
    ) -> Result<Global<Module>, String> {
        let contents = AvMod::load_file(path);
        let contents_str = v8::String::new(scope, &contents).unwrap();

        let json = v8::json::parse(scope, contents_str).unwrap();
        if scope.has_caught() {
            return Err("Error while parsing json!".into());
        }

        let export_names = [v8::String::new(scope, "default").unwrap()];

        let name = v8::String::new(scope, path.as_os_str().to_str().unwrap()).unwrap();
        let module = v8::Module::create_synthetic_module(scope, name, &export_names, Self::eval_callback); 
   
        let result = module.instantiate_module(scope, Self::_instantiate_callback);

        Ok(Global::new(scope, module))
    }

    fn _instantiate_callback<'a>(
        context: v8::Local<'a, v8::Context>,
        specifier: v8::Local<'a, v8::String>,
        import_assertions: v8::Local<'a, v8::FixedArray>,
        referrer: v8::Local<'a, v8::Module>,
    ) -> Option<v8::Local<'a, v8::Module>> {
        // ...
        println!("Instantiating JSON Module!");
        Some(referrer)
    }
} 