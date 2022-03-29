use v8::{HandleScope};

pub fn js_string<'a>(scope : &'a mut HandleScope, s : &'a str) -> v8::Local<'a, v8::String> {
    return v8::String::new(scope, s).unwrap();
}

pub fn js_number<'a>(scope : &'a mut HandleScope, n : f64) -> v8::Local<'a, v8::Number> {
    return v8::Number::new(scope, n);
}

pub fn js_func_on_object<'a>(scope : &'a mut HandleScope, obj : & v8::Local<'a, v8::Object>, name : v8::Local<'a, v8::String>, func : v8::Local<'a, v8::Function>) {
    obj.set(scope, name.into(), func.into()).unwrap();
}