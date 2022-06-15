// Custom Task Object

// Custom Events
// Cancellable

use std::{any::TypeId, thread, sync::mpsc::Sender};

use v8::{HandleScope, PromiseResolver, Global, Local, Value, Uint8Array, ArrayBuffer, Object, Exception, Function};

use crate::core::{def_safe_function, def_safe_property};

use super::{Runtime, message::{Message, Type as MessageType}, PromIndex};


const AUX_HANDLERS : &str = "___aux___";

pub struct Task {}
type RawOutput = Result<Vec<u8>, String>;

impl Task {
    pub fn new<'a, F>(scope: &mut HandleScope<'a>, type_id : TypeId, f: F) -> Local<'a, PromiseResolver>
        where
            F : FnOnce((PromIndex, Sender<Message>)) -> RawOutput + Send + 'static,
        {
        let prom = PromiseResolver::new(scope).unwrap();

        Self::assign_auxiliary_funcs(scope, prom.into());

        let global_prom = Global::new(scope, prom);

        let task_id = Runtime::prom_map_insert(scope, global_prom);
        
        let tx = Runtime::tx_from_scope(scope);

        thread::spawn(move || {
            let tx_client = tx.clone();
            let output = MessageType::Result(
                f((task_id, tx_client)), type_id
            ).message(task_id);
            tx.send(output).expect(format!("[📋 TASK {}] Failed to send its output.", task_id).as_str());
        });

        prom
    }

    // Add Task.on(event, hander)
    pub fn assign_auxiliary_funcs<'a>(scope: &mut HandleScope<'a>, obj : Local<'a, Object>) -> () {
        let blank_aux_handlers = Object::new(scope);
        def_safe_property(scope, obj, AUX_HANDLERS, blank_aux_handlers.into());
        def_safe_function!(scope, obj, "on", Self::on_callback);
    }

    pub fn on_callback<'a>(
        scope: &mut v8::HandleScope<'a>,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> () {
        if !args.get(0).is_string() {
            let msg = v8::String::new(scope, "Task.on's event parameter not provided (or not a string).").unwrap();
            let excp = v8::Exception::type_error(scope, msg);
            scope.throw_exception(excp.into());
        }

        if !args.get(1).is_function() {
            let msg = v8::String::new(scope, "Task.on's callback parameter not provided (or not a function).").unwrap();
            let excp = v8::Exception::type_error(scope, msg);
            scope.throw_exception(excp.into());
        }

        // Add the callback parameter to the 

        let event = args.get(0).to_rust_string_lossy(scope);
        let callback : Local<Function> = args.get(1).try_into().unwrap();

        let this = args.this();

        let handlers_key = v8::String::new(scope, AUX_HANDLERS).unwrap(); 
        let handlers : Local<Object> =
            this.get(scope, handlers_key.into()).expect("Task Handlers internal object is undefined !")
            .try_into().expect("Couldn't cast internal handlers to object!");

        let event_key = v8::String::new(scope, event.as_str()).unwrap();
        def_safe_property(scope, handlers, event.as_str(), callback.into());

        rv.set(this.into());
    }


    pub fn get_auxiliary_func<'a>(scope : &mut HandleScope<'a>, prom : &PromiseResolver, k : String) -> Option<Local<'a, Function>> {
        let aux_key = v8::String::new(scope, AUX_HANDLERS).unwrap();
        let handlers : Local<Object> = prom.get(scope, aux_key.into())
                .expect("Task should have handlers!")
                .try_into().expect("Auxiliary Handlers should be in an object!");
    
        let event_name = v8::String::new(scope, k.as_str()).unwrap(); 
        match handlers.get(scope, event_name.into()) {
            None => None,
            Some(v) => {
                match v.try_into() {
                    Ok(func) => Some(func),
                    Err(_) => None
                }
            }
        }
    }


    pub fn get_output<'a>(scope: &mut HandleScope<'a>,  vec : Vec<u8>, type_id : TypeId) -> Local<'a, Value> {
        
        match type_id {
            t if t == TypeId::of::<v8::String>() => {
                v8::String::new(scope, String::from_utf8(vec).unwrap().as_str()).unwrap().into()
            },
            t if t == TypeId::of::<v8::Uint8Array>() => {
                let len = vec.len();
                let store = ArrayBuffer::new_backing_store_from_boxed_slice(vec.into_boxed_slice());
                let int_arr = ArrayBuffer::with_backing_store(scope, &store.make_shared());
                Uint8Array::new(scope, int_arr, 0, len).unwrap().into()
            },
            _ => {
                v8::undefined(scope).into()
            }
        }
    }

}

pub mod Type {
    // Util for returning undefined
    pub struct Sink {}
}

pub use Type::Sink;