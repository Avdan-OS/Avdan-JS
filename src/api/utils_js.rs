use v8::{HandleScope};

pub fn js_string<'s>(scope : &mut HandleScope<'s>, s : & str) -> v8::Local<'s, v8::String> {
    return v8::String::new(scope, s).unwrap();
}

pub mod _Avdan {
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