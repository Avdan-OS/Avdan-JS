use std::{path::PathBuf, sync::mpsc::channel};

use colored::Colorize;
use v8::{TryCatch, HandleScope, Global, Module, ScriptOrigin, script_compiler::Source, CallbackScope, ModuleRequest, Value, ModuleStatus, Promise, Local, PromiseState};

use crate::Avdan::{utils, api::debug::AvDebug, runtime::avmod::AvModStore};

use super::{AvModProvider, AvMod, Specifier};

pub struct AvModJS {}

impl AvModJS {
    fn default_script_origin<'a, 'b>(
        scope: &mut HandleScope<'a>,
        name : &str,
        source_map_url: &str, 
    ) -> ScriptOrigin<'a> {
        let name = v8::String::new(scope, name).unwrap();
        let source_map_url = v8::String::new(scope, source_map_url).unwrap();
        ScriptOrigin::new(
            scope,
            name.into(),
            0i32,
            0i32,
            false,
            0i32,
            source_map_url.into(),
            false,
            false,
            true
        )
    }
}

impl AvModProvider for AvModJS {
    fn load_module<'a>(
        scope: &mut TryCatch<HandleScope<'a>>,
        path : &PathBuf
    ) -> Result<Global<Module>, String> {
        let source_text = v8::String::new(
            scope,
            AvMod::load_file(path).as_str()
        ).unwrap();

        let origin = AvModJS::default_script_origin(
            scope,
            path.file_name().unwrap().to_str().unwrap(),
            ""
        );

        let source_code = Source::new(source_text, Some(&origin));
        let module= match v8::script_compiler::compile_module(scope, source_code) {
            Some(s) => s,
            None => {
                let err = scope.exception().unwrap();
                let err = err.to_rust_string_lossy(scope);
                panic!("\n\t{} `{}`.\n\t{}", 
                    "Error compiling JavaScript in file".bright_red(), 
                    path.to_str().unwrap().yellow(),
                    err.bright_red()
                );
            }
        };


        // println!("");
        // println!("[{1}] {0}:", format!("Imports ({})", module.get_module_requests().length()).to_string().blue(), path.file_name().unwrap().to_str().unwrap().to_string().yellow());

        for import in utils::fixed_array_to_vec::<ModuleRequest>(scope, module.get_module_requests()) {
            let name = import.get_specifier();
            
            let res : Specifier = match name.to_rust_string_lossy(scope).try_into() {
                Ok(v) => v,
                Err(err) => panic!("{}", err)
            };   
            // println!("   {}\t{}", Colorize::bright_red("*").bold(), res);

            let dependency = AvMod::load(
                scope,
                &path.parent().unwrap().canonicalize().unwrap(), res.clone())?;


            
            let store  =scope.get_slot_mut::<AvModStore>().unwrap();
            store.register(res, dependency);
        }
        // println!("");


        match module.instantiate_module(
            scope, 
            Self::_instantiate_callback
        ) {
            None => {
                let excep = scope.exception().unwrap();
                let s= excep.to_rust_string_lossy(scope);
                panic!("Error from JS:\n\t{}\n", s.bright_red())
            },
            _ => {}
        };


        Ok(Global::new(scope, module))
    }

    fn _instantiate_callback<'a>(
        context: v8::Local<'a, v8::Context>,
        specifier: v8::Local<'a, v8::String>,
        import_assertions: v8::Local<'a, v8::FixedArray>,
        dependent: v8::Local<'a, v8::Module>,
    ) -> Option<v8::Local<'a, v8::Module>> {
        let scope = &mut unsafe {
            CallbackScope::new(context)
        };

        // println!("[{}] {}", specifier.to_rust_string_lossy(scope).yellow(), Colorize::blue("Callback from instantiation"), );
        
        // println!("[{}] Import assertions: {}", specifier.to_rust_string_lossy(scope).yellow(), import_assertions.length());
        
        let res : Specifier = specifier.to_rust_string_lossy(scope).try_into().unwrap();
        let store  =scope.get_slot_mut::<AvModStore>().unwrap();
        
        let sender = store.get_sender(&dependent.script_id().unwrap());
        let dependency = store.get(&res).expect("Could not find loaded module (!)");


        let s = &mut unsafe {
            CallbackScope::new(context)
        };
        
        Some(Local::new(s, dependency))
    }
}