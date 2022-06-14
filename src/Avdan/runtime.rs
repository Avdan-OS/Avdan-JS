use std::any::TypeId;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs;
use std::panic;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use rand::Rng;
use v8;
use v8::External;
use v8::Global;
use v8::HandleScope;
use v8::Local;
use v8::Promise;
use v8::PromiseResolver;
use v8::Value;
use v8::inspector::Channel;
use std::sync::mpsc::channel;

use crate::core::def_safe_property;
use crate::core::JSApi;
use crate::Avdan::loader::Extension;

use super::super::Avdan;

pub mod Task;

const TRANSMISSION_KEY: &str = "___TX___";
const PROMISE_TABLE: &str = "___PROM___";

type Index = u32; 
type Prom = Global<PromiseResolver>;

type TaskOut = (Index, Vec<u8>, TypeId);
type PromTable = HashMap<Index, Prom>;

pub struct Runtime<T> {
    tx: Option<Sender<T>>,
    tasks : Option<PromTable>,
}

impl Runtime<TaskOut> {
    pub fn new() -> Runtime<TaskOut> {
        Runtime {
            tx : None,
            tasks : Some(HashMap::new()),
        }
    }
    
    pub fn tx(&self) -> Sender<TaskOut> {
        self.tx.as_ref().expect("Err: tx is None!").clone()
    }

    pub fn run_extension(&mut self, args: Vec<String>) -> JoinHandle<()> {
        /*
         *     Extension Loader
         *  🚧 UNDER CONSTRUCTION 🚧
         */

        if args.len() < 2 {
            panic!("Extension path not specified!");
        }

        let extension = Extension::from_manifest(args.get(1).unwrap());
        
        /*
            Async ????
        */

        let (tx, rx) = channel();

        self.tx = tx.clone().into();

        
        thread::spawn(move || {
            let mut map : PromTable = HashMap::new();
            /*
             * V8 JavaScript (ECMAScript) Engine
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

                let avdan_js = Avdan::api::AvdanAPI {}.js(scope);

                def_safe_property(scope, global, "Avdan", avdan_js.into());

                

                let source_code =
                    fs::read_to_string(extension.main()).expect("Couldn't read `main` file!");

                // Create a string containing the JavaScript source code.
                let code = v8::String::new(scope, &source_code).unwrap();

                // Compile the source code.
                let script = v8::Script::compile(scope, code, None);

                // Put the async sender into the global JS scope. 
                let tx_ptr : *mut _= & mut tx.clone();
                let transmission = v8::External::new(scope, tx_ptr as *mut c_void);
                def_safe_property(scope, global, TRANSMISSION_KEY, transmission.into());

                // Put the promise map into the JS global scope.
                let map_ptr : *mut _ = &mut map;
                let prom_map = External::new(scope, map_ptr as *mut c_void);
                def_safe_property(scope, global, PROMISE_TABLE, prom_map.into());
                

                // Check if there was an error in the javascript
                // Run the script to get the result.
                script.expect("Error in the script!").run(scope).unwrap();

                for (id, contents, out_type) in rx {
                    println!("⏱️Task finished!");
                    println!("🤝🆔:\t{:?}", id);

                    // Get Promise, and resolve it, then remove from the table.
                    let p = map.get(&id).expect("Should have got promise !");
                    let prom = p.open(scope);



                    if out_type == TypeId::of::<v8::String>() {
                        let s = String::from_utf8(contents).unwrap();
                        // println!("Value: {}", s);
                        let st = v8::String::new(scope, s.as_str()).unwrap();

                        prom.resolve(scope, st.into());
                    } else {
                        let u = v8::undefined(scope).into();
                        prom.resolve(scope, u);
                    }

                    map.remove(&id);

                    if map.len() == 0 {
                        break;
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
        let key =  v8::String::new(scope, TRANSMISSION_KEY).unwrap();
        let global = scope.get_current_context().global(scope);

        let __tx : Result<Local<External>, _> = global.get(scope, key.into()).unwrap().try_into();
        let _tx = __tx.expect("Cannot cast tx into v8::External !");

        let tx = _tx.value() as *mut Sender<TaskOut>;

        unsafe { tx.as_mut() }.expect("Cannot change as mut!").to_owned()
    }

    pub fn prom_map_insert<'a>(scope: &mut HandleScope<'a>, prom: Prom) -> Index {
        let key = v8::String::new(scope, PROMISE_TABLE).unwrap();
        let global = scope.get_current_context().global(scope);

        let ___tbl : Result<Local<External>, _> = global.get(scope, key.into()).unwrap().try_into();
        let __tbl = ___tbl.expect("Cannot cast prom_tbl into v8::External !");
        let _tbl = __tbl.value() as *mut HashMap<Index, Prom>;

        let tbl = unsafe {_tbl.as_mut()}.expect("Cannot change to mut !");

        let mut rng = rand::thread_rng();
        let mut i = 0u32;

        while tbl.contains_key(&i) {
            i = rng.gen_range(u32::MIN..u32::MAX)
        }

        tbl.insert(i, prom);
        println!("🤝 Added promise #{:?} to table !", i);
        return i;
    }
}
