use v8::{HandleScope, Local, Object, Value, };

pub trait JSApi {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object>;
}

pub fn def_safe_property<'a>(scope: &mut HandleScope<'a>, obj: Local<Object>, name: &str, val: Local<Value>) -> () {
    let k = v8::String::new(scope, name).unwrap();
    obj.define_own_property(scope, k.into(), val, v8::READ_ONLY);
}

mod macros {
    macro_rules! def_safe_function {
        ($_scope: ident, $_obj:expr, $_name: expr, $func: expr) => {{{
            let name: &str = $_name;
            let scope: &mut v8::HandleScope = $_scope;
            let obj : v8::Local<v8::Object> = $_obj; 

            let f = v8::FunctionBuilder::<v8::Function>::new($func).build(scope).unwrap();
            let k = v8::String::new(scope, name).unwrap();
            obj.define_own_property(scope, k.into(), f.into(), v8::READ_ONLY);
        }}}
    }
    pub(crate) use def_safe_function;
}


pub use macros::*;