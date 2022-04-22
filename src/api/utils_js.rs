use v8::{HandleScope};

pub fn js_string<'s>(scope : &mut HandleScope<'s>, s : & str) -> v8::Local<'s, v8::String> {
    return v8::String::new(scope, s).unwrap();
}

pub fn js_number<'a>(scope : &mut HandleScope<'a>, n : f64) -> v8::Local<'a, v8::Number> {
    return v8::Number::new(scope, n);
}

pub fn js_func_on_object<'a, 'b>(scope : &mut HandleScope<'a>, obj : & v8::Local<'b, v8::Object>, name : v8::Local<'b, v8::String>, func : v8::Local<'b, v8::Function>) {
    obj.set(scope, name.into(), func.into()).unwrap();
}

pub fn set_func(
    scope: &mut v8::HandleScope<'_>,
    obj: v8::Local<v8::Object>,
    name: &'static str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
  ) {
    let key = v8::String::new(scope, name).unwrap();
    let tmpl = v8::FunctionTemplate::new(scope, callback);
    let val = tmpl.get_function(scope).unwrap();
    val.set_name(key);
    obj.set(scope, key.into(), val.into());
  }


pub mod Avdan {
    pub struct Error {
        message : String,
        code : String,
    }
    
    impl Error {
        pub fn new(code : String, message : String) -> Error {
            return Error {code, message};
        }

        pub fn str(code : &str, message : &str) -> Error {
            return Error {code : String::from(code), message: String::from(message)};
        }

        pub fn to_js<'s>(&self, _scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::Value> {
            let message = v8::String::new(_scope, format!("#{} - {}", self.code, self.message).as_str()).unwrap(); 
            let err = v8::Exception::error(_scope, message);
            let error = err.to_object(_scope).unwrap();

            let code_key = v8::String::new(_scope, "code").unwrap();
            let error_code = v8::String::new(_scope, &self.code.to_string()).unwrap();
            error.set(_scope, code_key.into(), error_code.into());
            
            return error.into();
        }
    }

}