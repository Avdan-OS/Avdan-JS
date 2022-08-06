use colored::Colorize;
use v8::{HandleScope, Global, Module, TryCatch, Local, Context, Value, CallbackScope};

use crate::Avdan::{api::AvdanAPI, utils};

use super::{Specifier, AvModStore};

pub struct AvModInternal {}

impl AvModInternal {
    pub fn get_internal_module<'a>(scope: &mut HandleScope<'a>, name: String) -> Result<Global<Module>, String> {
        let apis = AvdanAPI::public_apis();

        match apis.get(name.as_str()) {
            Some(api) => {
                let obj = api.as_ref().js(scope);
                let mut export_names = vec![v8::String::new(scope, "default").unwrap()];
                let names = obj.get_own_property_names(scope).unwrap();

                for name in utils::array_to_vec::<v8::String>(scope, names) {
                    export_names.push(name);
                }
                
                let _name = name.clone();
                let name  = v8::String::new(scope, name.as_str()).unwrap();
                let scope = &mut TryCatch::new(scope);

                let module = Module::create_synthetic_module(
                    scope,
                    name,
                    export_names.as_slice(),
                    Self::evaluation_steps
                );

                {
                    let g = Global::new(scope, module);
                    let store  =scope.get_slot_mut::<AvModStore>().unwrap();
                    store.add_internal(g, _name);
                }

                match module.instantiate_module(scope, Self::instantiate_callback) {
                    None => {
                        let excep = scope.exception().unwrap();
                        Err(format!("Error from JS:\n\t{}", excep.to_rust_string_lossy(scope).as_str().bright_red()))
                    },
                    
                    Some(r) => Ok(Global::new(scope, module))
                }
            },
            
            None => Err(format!("{}{} {}", "internal module @avdan/".bright_red(), name.as_str().yellow(), "not found!".bright_red()))
        }
    }

    fn instantiate_callback<'a> (
        context           : v8::Local<'a, v8::Context>,
        specifier         : v8::Local<'a, v8::String>,
        import_assertions : v8::Local<'a, v8::FixedArray>,
        referrer          : v8::Local<'a, v8::Module>,
     ) -> Option<v8::Local<'a, v8::Module>> {
        // TODO: Resolve properly
        Some(referrer)
    }

    fn evaluation_steps<'a>(
        ctx: Local<'a, Context>,
        module: Local<'a, Module>
    ) -> Option<Local<'a, Value>> {
        let scope = &mut unsafe {
            CallbackScope::new(ctx)
        };

        let g = Global::new(scope, module);

        let store  = scope.get_slot_mut::<AvModStore>().unwrap();

        let s = &mut unsafe {
            CallbackScope::new(ctx)
        };
        
        let try_catch = &mut TryCatch::new(s);

        let name = store.get_internal(&g).expect("Expected same name!");

        // let name = "clipboard".to_string();
        let apis = AvdanAPI::public_apis();
        let api = apis.get(name.as_str()).unwrap();

        let obj = api.js(try_catch);

        let keys = obj.get_own_property_names(try_catch).unwrap();

        for key in utils::array_to_vec::<v8::String>(try_catch, keys) {
            let value = obj.get(try_catch, key.into()).unwrap();
            let res = module.set_synthetic_module_export(
                try_catch,
                key,
                value
            );

            match res {
                Some(_) => {},
                None => {
                    let excep =try_catch.exception().unwrap();
                    panic!("Error from JS:\n\t{}\n", excep.to_rust_string_lossy(scope).bright_red());
                }
            }
        } 

        let default = v8::String::new(try_catch, "default").unwrap();
        
        match module.set_synthetic_module_export (
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
