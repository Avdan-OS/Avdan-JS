use std::slice;

use v8::{HandleScope, Local, Object, Value, Uint8Array, ArrayBuffer};

use crate::core::{JSApi, AvJSObject, def_safe_function};

/*
    A collection of network functions.
    Should contain Fetch API
*/

enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

// Default to GET if name not valid.

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "head" => Self::HEAD,
            "post" => Self::POST,
            "put" => Self::PUT,
            "delete" => Self::DELETE,
            "connect" => Self::CONNECT,
            "options" => Self::OPTIONS,
            "trace" => Self::TRACE,
            "patch" => Self::PATCH,
            _ => Self::GET,
        }
    }
}

impl From<Method> for reqwest::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::CONNECT => Self::CONNECT,
            Method::DELETE => Self::DELETE,
            Method::GET => Self::GET,
            Method::HEAD => Self::HEAD,
            Method::OPTIONS => Self::OPTIONS,
            Method::PATCH => Self::PATCH,
            Method::POST => Self::POST,
            Method::PUT => Self::PUT,
            Method::TRACE => Self::TRACE
        }
    }
}

impl AvJSObject for Method {
    fn deserialize<'a>(scope: &mut HandleScope<'a>, obj: Local<Value>) -> Result<Self, String> 
        where Self: Sized {
            if !obj.is_string() {
                return Err("Method value is not string !".to_string());
            }

            let contents = obj.to_rust_string_lossy(scope);
            Ok(Method::from(contents))
    }

    fn serialize<'a>(&self, scope: &mut HandleScope<'a>) -> Result<Local<'a, Value>, String> { unimplemented!() }
}



enum Body {
    Bytes(Vec<u8>),
    Text(String),
    // Todo stream, or pipe
}

impl AvJSObject for Body {
    fn deserialize<'a>(scope: &mut HandleScope<'a>, obj: Local<Value>) -> Result<Self, String> 
        where Self: Sized {
        if obj.is_uint8_array() {
            let obj: Local<Uint8Array> = obj.try_into().unwrap();
            let arr = obj.buffer(scope).unwrap();
            let len = obj.byte_length();
            let data = arr.get_backing_store().data().unwrap().as_ptr() as *const u8;


            let vec = unsafe {
                slice::from_raw_parts(data, len)
            }.to_vec();

            return Ok(Self::Bytes(vec));
        }

        if obj.is_string() {
            let string : Local<v8::String> = obj.try_into().unwrap();

            return Ok(Self::Text(string.to_rust_string_lossy(scope)));
        }

        Err("Body must be either Uint8Array (bytes) or string!".to_string())

    }

    fn serialize<'a>(&self, scope: &mut HandleScope<'a>) -> Result<Local<'a, Value>, String> {
        unimplemented!();
    }
}


impl From<Body> for reqwest::blocking::Body {
    fn from(b: Body) -> Self {
        match b {
            Body::Bytes(b) => Self::from(b),
            Body::Text(t) => Self::from(t),
        }
    }
}


// Simplified version of the fetch API
mod fetch;
use fetch::Fetch;
pub struct AvNet;

impl AvNet {
    fn assign_functions<'a>(scope: &mut HandleScope<'a>, obj : Local<Object>) -> () {
        def_safe_function!(scope, obj, "fetch", Fetch::fetch);
    }
}

impl JSApi for AvNet {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        let obj = Object::new(scope);
        Self::assign_functions(scope, obj);
        obj
    }

    
}