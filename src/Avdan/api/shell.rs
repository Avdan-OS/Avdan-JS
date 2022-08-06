use std::io::{Read, self, Write};
use std::process::{Command, Stdio};

use avdanos_search_macros::permission;
use v8::{HandleScope, Local, Object, FunctionCallbackArguments, ReturnValue};

use crate::core::{JSApi, def_safe_function};
use crate::Avdan;
pub struct AvShell {}

impl AvShell {
    // Shell.exec(cmd : string, ...args : string[]) -> ShellResult
    #[permission(avdan.shell.exec)]
    pub fn exec (
        scope  : &mut HandleScope,
        args   : FunctionCallbackArguments,
        mut rv : ReturnValue
    ) -> () {
        if args.length() == 0 {
            let msg = v8::String::new(scope,"No args provided!").unwrap();
            let exception = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exception);
            
            return;
        }

        let mut cmd_args : Vec<String> = Vec::new();

        for i in 0..args.length() {
            if !args.get(i).is_string() {
                let msg = v8::String::new(scope,"Args must all be strings!").unwrap();
                let exception = v8::Exception::type_error(scope, msg);
                scope.throw_exception(exception);
                
                return;
            }
            
            cmd_args.push(args.get(i).to_rust_string_lossy(scope));
        }

        rv.set(v8::undefined(scope).into());
    }

    fn assign_functions<'a> (
        scope : &mut HandleScope<'a>,
        obj   : Local<Object>
    ) -> () {
        def_safe_function!(scope, obj, "exec", Self::exec);
    }
}

impl JSApi for AvShell {
    fn js<'a> (
        &self, 
        scope: &mut v8::HandleScope<'a>
    ) -> Local<'a, Object> {
        let obj = Object::new(scope);
        Self::assign_functions(scope, obj);
        
        obj
    }
}

struct ShellObj {
    args : Vec<String>
}

impl ShellObj {
    fn execute(&self) -> Result<(), ()> {
        Err(())
    }
}
