use std::collections::HashMap;
use std::ffi::c_void;
use std::intrinsics::{transmute,};
use std::mem::size_of_val;
use std::ptr;
use std::slice::{from_raw_parts, self};
use std::str::FromStr;
use avdanos_search_macros::permission;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use v8::{Local, Object, Value, Exception, HandleScope};
use crate::Avdan;
use crate::Avdan::runtime::{Task, output};
use crate::core::obj_has_property;
use crate::{core::{AvJSObject, obj_get_property}, Avdan::utils::array_to_vec};

use super::{Body, Method};

struct Options {
    method  : Method,
    headers : Option<HashMap<String, String>>,
    body    : Option<Body>,
}

impl AvJSObject for HashMap<String, String> {
    fn deserialize<'a>(scope: &mut v8::HandleScope<'a>, obj: Local<Value>) -> Result<Self, String> 
        where Self: Sized {
        if !obj.is_object() {
            return Err("Not object!".to_string());
        }

        let obj : Local<Object> = obj.try_into().unwrap();

        let keys = obj.get_own_property_names(scope).unwrap();
    
        let mut res = HashMap::<String,String>::new();

        for key in array_to_vec::<Value>(scope, keys) {
            let value = obj.get(scope, key).unwrap();

            if !key.is_string() || !value.is_string() {
                return Err("Member of object not a string !".to_string());
            }


            let k = key.to_rust_string_lossy(scope);
            let v = value.to_rust_string_lossy(scope);

            res.insert(k, v);
        }

        return Ok(res);
    }

    fn serialize<'a>(&self, scope: &mut v8::HandleScope<'a>) -> Result<Local<'a, Value>, String> {
        let res = Object::new(scope);

        for (k, v) in self.iter() {
            let key = v8::String::new(scope, k).unwrap();
            let value = v8::String::new(scope, v).unwrap();

            res.set(scope, key.into(), value.into());
        }

        Ok(res.into())
    }
}


struct Response;

impl Response {
    fn from_prom_callback<'a>(scope: &mut HandleScope<'a>, mut vec : Vec<u8>) -> Local<'a, Value> {

        let s= vec.as_mut_slice();

        // let a = unsafe {
        //     ptr::read(s as *mut [u8] as *mut reqwest::blocking::Response)
        // };

        // let txt = a.text().unwrap();

        // let string = v8::String::new(scope, txt.as_str()).unwrap();
        let udef = v8::undefined(scope);
        udef.into()
    }
}


trait IntoHeaders {
    fn into_headers(self) -> Result<HeaderMap<HeaderValue>, String>; 
}

impl IntoHeaders for HashMap<String, String> {
    fn into_headers(self) -> Result<HeaderMap<HeaderValue>, String> {
        let mut h = HeaderMap::with_capacity(self.len());
        // TODO: Better error handling...
        for (name, value) in self.iter() {
            h.append(
                match HeaderName::from_str(name) {
                    Ok(h) => h,
                    Err(e) => return Err(format!("{}", e)),
                }, 
                match HeaderValue::from_str(value) {
                    Ok(v) => v,
                    Err(e) => return Err(format!("{}", e)),
                });
        }
        Ok(h)
    }
}

impl AvJSObject for Options {
    fn deserialize<'a>(scope: &mut v8::HandleScope<'a>, obj: Local<Value>) -> Result<Self, String> 
        where Self: Sized {
            let mut s = Self {
                method: Method::GET,
                headers: None,
                body: None,
            };

            if !obj.is_object() {
                return Err(format!("Options must be an object !"));
            }


            let obj : Local<Object> = obj.try_into().unwrap();

            if obj_has_property!(scope, obj, "method") {
                let m = obj_get_property(scope, obj, "method");
                s.method = Method::deserialize(scope, m).unwrap_or_else(|_| Method::GET).into();
            }

            if obj_has_property!(scope, obj, "headers") {
                let m = obj_get_property(scope, obj, "headers");
                s.headers = HashMap::<String, String>::deserialize(scope, m).ok();
            }

            if obj_has_property!(scope, obj, "body") {
                let m = obj_get_property(scope, obj, "body");
                s.body = Body::deserialize(scope, m).ok();
            }
            Ok(s)
    }

    fn serialize<'a>(&self, scope: &mut v8::HandleScope<'a>) -> Result<Local<'a, Value>, String> {
        unimplemented!()
    }
}

pub struct Fetch;

impl Fetch {
    #[permission(avdan.net.fetch)]
    pub fn fetch<'a>(
        scope: &mut v8::HandleScope<'a>,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> () {

        let udef = v8::undefined(scope);


        let uri = args.get(0);

        if !uri.is_string() {
            let s = v8::String::new(scope, "URI must be string !").unwrap();
            let excep = Exception::type_error(scope, s);
            scope.throw_exception(excep);
        }

        let opts = args.get(1);
        let opts = Options::deserialize(scope, opts);
        
        let opts = match opts {
            Ok(opts) => opts,
            Err(err) => {
                let s = v8::String::new(scope, format!("Options not structured properly:\n{}", err).as_str()).unwrap();
                let excep = Exception::type_error(scope, s);
                scope.throw_exception(excep);

                rv.set(udef.into());
                return;
            },
        };

        let uri = uri.to_rust_string_lossy(scope);

        let prom = Task::new(
            scope, 
            move |(id, tx)| {

                println!("{}", "#1".blue());
                let client = reqwest::blocking::Client::new();


                let r = client.request(
                    reqwest::Method::from(opts.method), uri);

                let headers = opts.headers.unwrap_or(HashMap::new())
                    .into_headers()?;

                let mut r = r.headers(headers);

                if opts.body.is_some() {
                    let b : reqwest::blocking::Body = opts.body.unwrap().into();
                    r = r.body(b);

                }

                println!("{}", "#2".blue());

                let res = r.send();
                
                match res {
                    Ok(mut res) => {
                        res.copy_to(w)
                        let c = v;
                        Ok(c.to_vec())
                    },
                    Err(e) => {
                        Err(e.to_string())
                    }
                }
            },
            Response::from_prom_callback
        );


        rv.set(prom.into());
    }
}