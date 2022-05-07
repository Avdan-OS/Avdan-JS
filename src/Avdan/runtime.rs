use v8;
use std::fs;
use std::panic;

use crate::Avdan::loader::Extension;
use crate::core::JSApi;
use crate::core::def_safe_property;

use super::super::Avdan;


pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }
    pub fn run_extension(&self, args : Vec<String>) -> () {

        /*
         *     Extension Loader 
         *  🚧 UNDER CONSTRUCTION 🚧
         */

    
        if args.len() < 2 {
            panic!("Extension path not specified!");
        }

        let extension  = Extension::from_manifest(args.get(1).unwrap());

    
        /*     
         * JavaScript (ECMAScript) Engine 
         */
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

    {
        // Create a new Isolate and make it the current one.
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        let handle_scope = &mut v8::HandleScope::new(isolate);
        

        // Create a new context.
        let context = v8::Context::new(handle_scope);


        // Enter the context for script compilation and execution
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Make a global scope thing-y
        let global = context.global(scope);
        
        /*
         *     Security Policy 
         * 🚧 UNDER CONSTRUCTION 🚧
        */        

        // Apply security policy
        extension.security().into_scope(scope);

        // Test secure set
        
        /*
         *  Avdan API  
         */

        // Avdan Global Interface
        // let key_avdan = v8::String::new(scope, "Avdan").unwrap();
        
        // The global Avdan Search API object.
        // let avdan_obj = Avdan::api::AvdanAPI {};
        
        // // Avdan.Debug API
        // {
        //     let debug = v8::Object::new(scope);

        //     // Avdan.Debug.log
        //     {
        //         let label = v8::String::new(scope, "log").unwrap();
        //         let func = v8::FunctionBuilder::<v8::Function>::new(debug_bind::log).build(scope).unwrap();
            
        //         debug.set(scope, label.into(), func.into());
        //     }

        //     // Avdan.Debug
        //     let label = v8::String::new(scope, "Debug").unwrap();
            
        //     avdan_obj.set(
        //         scope,
        //         label.into(),
        //         debug.into()
        //     );
        // }

        // // Avdan.File API 
        // {
        //     let file_api = api::file::AvFile::new().js(scope);

        //     let file_label = v8::String::new(scope, "File").unwrap();
            
        //     avdan_obj.set(
        //         scope,
        //         file_label.into(),
        //         file_api.into()
        //     );
        // }

        // // // Avdan.Clipboard API
        // {
        //     let clipboard = clipboard::Clip::JS(scope);

        //     // Avdan.Clipboard
        //     let clipboard_label = v8::String::new(scope, "Clipboard").unwrap();
            
        //     avdan_obj.set(
        //         scope,
        //         clipboard_label.into(),
        //         clipboard.into()
        //     );
        // }
        
        let avdan_js = Avdan::api::AvdanAPI{}.js(scope);

        def_safe_property(scope, global, "Avdan", avdan_js.into());
        
        let source_code = fs::read_to_string(extension.main()).expect("Couldn't read `main` file!");

        // Create a string containing the JavaScript source code.
        let code = v8::String::new(scope, &source_code).unwrap();


        // Compile the source code.
        let script = v8::Script::compile(scope, code, None);

        // Check if there was an error in the javascript
        if script.is_some() {
            // Run the script to get the result.
            script.unwrap().run(scope).unwrap();
        }

    }
    unsafe {
        v8::V8::dispose();
    }

    v8::V8::dispose_platform();
    }
}