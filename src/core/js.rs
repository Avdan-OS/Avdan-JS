use v8::{HandleScope, Local, Object, Value,};

pub trait JSApi {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object>;
}

pub trait AvJSObject {
    fn deserialize<'a>(scope: &mut HandleScope<'a>, obj: Local<Value>) -> Result<Self, String> 
        where Self: Sized;
    
    fn serialize<'a>(&self, scope : &mut HandleScope<'a>) -> Result<Local<'a, Value>, String>;
}

pub fn def_safe_property<'a>(scope: &mut HandleScope<'a>, obj: Local<Object>, name: &str, val: Local<Value>) -> () {
    let k = v8::String::new(scope, name).unwrap();
    
    obj.define_own_property(scope, k.into(), val, v8::READ_ONLY);
}

pub fn obj_get_property<'a>(scope: &mut HandleScope<'a>, obj: Local<Object>, name: &str) -> Local<'a, Value> {
    let k = v8::String::new(scope, name).unwrap();
    obj.get(scope, k.into()).unwrap()
}

mod macros {
    macro_rules! def_safe_function {
        ($_scope: ident, $_obj: expr, $_name: expr, $func: expr) => {{{
            let name: &str = $_name;
            let scope: &mut v8::HandleScope = $_scope;
            let obj : v8::Local<v8::Object> = $_obj; 

            let f = v8::FunctionBuilder::<v8::Function>::new($func).build(scope).unwrap();
            let k = v8::String::new(scope, name).unwrap();
            
            obj.define_own_property(scope, k.into(), f.into(), v8::READ_ONLY);
        }}}
    }

    macro_rules! obj_has_property {
        ($_scope: ident, $_obj: expr, $_name: expr) => {{{
            let name: &str = $_name;
            let scope: &mut v8::HandleScope = $_scope;
            let obj : v8::Local<v8::Object> = $_obj; 

            let k = v8::String::new(scope, name).unwrap();
            
            obj.has_own_property(scope, k.into()).unwrap()
        }}};
    }
    
    pub(crate) use def_safe_function;
    pub(crate) use obj_has_property;
}

pub use macros::*;
