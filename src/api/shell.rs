use v8::{Object, HandleScope, Local, ObjectTemplate};



struct Shell {}

impl Shell {
    pub fn js<'s>(scope : &mut HandleScope<'s>) -> Local<'s, Object> {
        let mut file_api = ObjectTemplate::new(scope);

        // Avdan.Shell <>
        ShellJS::assign_functions(scope, &mut file_api);

        // Avdan.Shell </>

        file_api.new_instance(scope).unwrap()
    }
}

struct ShellJS {}

impl ShellJS {
    fn assign_functions<'s>(
        scope: &mut v8::HandleScope<'s>,
        obj: &mut v8::Local<'s, v8::ObjectTemplate>
    ) -> () {

    }

    pub fn exec(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> () {
        
    }
}