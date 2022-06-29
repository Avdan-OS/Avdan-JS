use colored::Colorize;
use rand::Rng;
use v8::PromiseRejectMessage;
use std::any::TypeId;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::env;
use std::ffi::c_void;
use std::fs;
use std::intrinsics::transmute;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use v8;
use v8::inspector::Channel;
use v8::CallbackScope;
use v8::Context;
use v8::External;
use v8::FixedArray;
use v8::Global;
use v8::HandleScope;
use v8::Local;
use v8::Module;
use v8::Promise;
use v8::PromiseResolver;
use v8::ScriptOrigin;
use v8::TryCatch;
use v8::Value;

use crate::Avdan::runtime::avmod::AvModJS;
use crate::Avdan::runtime::avmod::AvModProvider;
use crate::Avdan::runtime::avmod::AvModStore;
use crate::core::def_safe_property;
use crate::core::JSApi;
use crate::Avdan::loader::Extension;

use super::super::Avdan;

pub mod task;
pub use task::{output, Task};

pub mod avmod;

pub mod message;
pub use message::{Builder, Message, Type};

const TRANSMISSION_KEY: &str = "___TX___";
const PROMISE_TABLE: &str = "___PROM___";

pub type PromIndex = u32;
type Prom = Global<PromiseResolver>;

type TaskOut = Message;
type PromTable = HashMap<PromIndex, Prom>;

pub struct Runtime<T> {
    tx: Option<Sender<T>>,
}

impl Runtime<TaskOut> {
    pub fn new() -> Runtime<TaskOut> {
        Runtime { tx: None }
    }

    pub fn tx(&self) -> Sender<TaskOut> {
        self.tx.as_ref().expect("Err: tx is None!").clone()
    }

    extern "C" fn promise_reject_callback<'a>(msg : PromiseRejectMessage<'a>) -> () {
        let scope = &mut unsafe { v8::CallbackScope::new(&msg) };
        let v = msg.get_value().unwrap();
        let s = v.to_rust_string_lossy(scope);
        println!("\n{}\n{}", "Uncaught error in JS!".red(), s.bright_red());
        exit(1);
    }

    pub fn run_extension(&mut self, args: Vec<String>) -> JoinHandle<()> {
        /*
         *     Extension Loader
         *  🚧 UNDER CONSTRUCTION 🚧
         */

        if args.len() < 2 {
            panic!("Extension path not specified!");
        }

        let experiental_module_flag = match args.get(2) {
            Some(f) => f.eq("--module"),
            None => false,
        };

        let extension = Extension::from_manifest(args.get(1).clone().unwrap());

        /*
            Async ????
        */

        let (tx, rx) = channel();

        self.tx = tx.clone().into();

        thread::spawn(move || {
            let mut map: PromTable = HashMap::new();
            /*
             * V8 JavaScript (ECMAScript) Engine
             */
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::set_flags_from_string("--harmony-import-assertions");
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

                let avdan_js = Avdan::api::AvdanAPI {}.js(scope);

                def_safe_property(scope, global, "Avdan", avdan_js.into());

                // TODO: Move this
                // {
                // Put the async sender into the global JS scope.
                let tx_ptr: *mut _ = &mut tx.clone();
                let transmission = v8::External::new(scope, tx_ptr as *mut c_void);
                def_safe_property(scope, global, TRANSMISSION_KEY, transmission.into());

                // Put the promise map into the JS global scope.
                let map_ptr: *mut _ = &mut map;
                let prom_map = External::new(scope, map_ptr as *mut c_void);
                def_safe_property(scope, global, PROMISE_TABLE, prom_map.into());
                // }

                
                scope.set_promise_reject_callback(Self::promise_reject_callback);

                if experiental_module_flag {
                    let exp_warning_message = Colorize::yellow("Warning! --module is an experimental flag!\n");
                    
                    println!("{}\n Do not expect anything to work !", exp_warning_message);
    
                    
                    let scope = &mut v8::HandleScope::new(scope);
                    let try_catch = &mut TryCatch::new(scope);
                    AvModStore::into_scope(try_catch);

                    let main_module_path = Path::new(extension.main());
                    let main_module = AvModJS::load_module(
                        try_catch,
                        &main_module_path.canonicalize().unwrap()
                    );

                    let main = match main_module {
                        Ok(module) => module,
                        Err(err) => panic!("\n\n\t{}:\n\t\t{}\n\n", "Error".bright_red(), err)
                    };

                    let m = main.open(try_catch);
                    
                    let a : Local<Promise> = m.evaluate(try_catch).unwrap().try_into().expect("Should be promise!");

                    // let main_module = main_module.resolve(
                    //     scope,
                    //     env::current_dir().unwrap().to_str().unwrap().to_string(),
                    // ).unwrap();

                    // let main = main_module
                    //     .open(scope);
                    

                    // main.evaluate(scope).unwrap();


                    // Check if there was an error in the javascript
                    // Run the script to get the result.
                } else {
                    // Compile the source code.
                    let source_code = fs::read_to_string(extension.main()).unwrap();
                    let source_code = v8::String::new(scope, &source_code).unwrap();
                    let script = v8::Script::compile(scope, source_code, None);

                    script.expect("Error in the script!").run(scope).unwrap();
                }

                if map.len() != 0 {
                    for msg in rx {
                        // Very simplified event loop.

                        let id = msg.0;

                        let p = map.get(&id).expect("Should have got promise !");
                        let prom = p.open(scope);

                        match msg.1 {
                            Type::Auxiliary(k, contents, fn_ptr) => {
                                match Task::get_auxiliary_func(scope, prom, k) {
                                    Some(f) => {
                                        let obj = fn_ptr(scope, contents);
                                        let local = unsafe {
                                            transmute::<&PromiseResolver, Local<PromiseResolver>>(
                                                prom,
                                            )
                                        };
                                        f.call(scope, local.into(), &[obj]);
                                    }
                                    None => {}
                                }
                            }
                            Type::Result(contents, builder) => {
                                // Get Promise, and resolve it, then remove from the table.
                                match contents {
                                    Err(txt) => {
                                        let e = v8::String::new(scope, &txt).unwrap();
                                        let err = v8::Exception::error(scope, e);
                                        prom.reject(scope, err.into());
                                    }
                                    Ok(result) => {
                                        let r_value = builder(scope, result);
                                        prom.resolve(scope, r_value);
                                    }
                                }
                                map.remove(&id);
                            }
                        };

                        if map.len() == 0 {
                            break;
                        }
                    }
                }
            }

            unsafe {
                v8::V8::dispose();
            }

            v8::V8::dispose_platform();
        })
    }

    pub fn tx_from_scope<'a>(scope: &mut HandleScope<'a>) -> Sender<TaskOut> {
        let key = v8::String::new(scope, TRANSMISSION_KEY).unwrap();
        let global = scope.get_current_context().global(scope);

        let __tx: Result<Local<External>, _> = global.get(scope, key.into()).unwrap().try_into();
        let _tx = __tx.expect("Cannot cast tx into v8::External !");

        let tx = _tx.value() as *mut Sender<TaskOut>;

        unsafe { tx.as_mut() }
            .expect("Cannot change as mut!")
            .to_owned()
    }

    pub fn prom_map_insert<'a>(scope: &mut HandleScope<'a>, prom: Prom) -> PromIndex {
        let key = v8::String::new(scope, PROMISE_TABLE).unwrap();
        let global = scope.get_current_context().global(scope);

        let ___tbl: Result<Local<External>, _> = global.get(scope, key.into()).unwrap().try_into();
        let __tbl = ___tbl.expect("Cannot cast prom_tbl into v8::External !");
        let _tbl = __tbl.value() as *mut HashMap<PromIndex, Prom>;

        let tbl = unsafe { _tbl.as_mut() }.expect("Cannot change to mut !");

        let mut rng = rand::thread_rng();
        let mut i = 0u32;

        while tbl.contains_key(&i) {
            i = rng.gen_range(u32::MIN..u32::MAX)
        }

        tbl.insert(i, prom);
        return i;
    }
}
