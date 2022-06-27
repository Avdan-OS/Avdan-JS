use std::intrinsics::transmute;

use colored::Colorize;
use v8::{Exception, Local, Value, Global, Module, Context, HandleScope, CallbackScope, TryCatch, Object};

use crate::Avdan::utils;

use super::{AvModProvider, AvMod, AvModStore};

pub struct AvModJSON {}

impl AvModJSON {
    fn evaluation_steps<'a>(
        ctx: Local<'a, Context>,
        module: Local<'a, Module>
    ) -> Option<Local<'a, Value>> {
        let scope = &mut unsafe {
            CallbackScope::new(ctx)
        };

        let g = Global::new(scope, module);
        let store  = scope.get_slot_mut::<AvModStore>().unwrap();
        let obj = store.get_json(&g).unwrap();


        let s = &mut unsafe {
            CallbackScope::new(ctx)
        };
        let try_catch = &mut TryCatch::new(s);

        let obj = Local::new(try_catch, obj);

        let default = v8::String::new(try_catch, "default").unwrap();
        
        match module.set_synthetic_module_export(
            try_catch,
            default,
            obj.into()
        ) {
            Some(_) => {},
            None => {
                let excep =try_catch.exception().unwrap();
                panic!("Error from JS:\n\t{}\n", excep.to_rust_string_lossy(try_catch).bright_red());
            }
        }

        Some(v8::undefined(try_catch).into())
    }
}

impl AvModProvider for AvModJSON {
    fn load_module<'a>(
        scope: &mut v8::TryCatch<v8::HandleScope<'a>>,
        path : &std::path::PathBuf
    ) -> Result<Global<Module>, String> {
        let contents = AvMod::load_file(path);
        let contents_str = v8::String::new(scope, &contents).unwrap();

        let json : Local<Object> = v8::json::parse(scope, contents_str).unwrap().try_into().unwrap();
        if scope.has_caught() {
            return Err("Error while parsing json!".into());
        }

        

        let export_names = [v8::String::new(scope, "default").unwrap()];

        let name = v8::String::new(scope, path.as_os_str().to_str().unwrap()).unwrap();
        let module = v8::Module::create_synthetic_module(scope, name, &export_names, Self::evaluation_steps); 
   
        {
            let g = Global::new(scope, module);
            let g_json  = Global::new(scope, json);
            let store  =scope.get_slot_mut::<AvModStore>().unwrap();
            store.add_json(g, g_json);
        }



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
        Some(referrer)
    }


} 