use std::{ffi::c_void, mem};
use serde::{Serialize, Deserialize};

use super::Permission;
#[derive(Clone, Serialize, Deserialize)]
pub struct Constraints {
    permissions : Vec<String>,
    commands: Vec<String>,
}

const SECURITY_KEY : &str = "___SECURITY___";

/**
 * Security::Constraints
 * 
 * Handles various security contraints of AvdanOS Search extensions, such as:
 *      *    Shell command declararion.
 *      *    AvdanOS API access. 
 * 
 */
impl Constraints {
    /** "CONSTRUCTOR" */
    pub fn new<'a>(permissions : Vec<&'a str>, external_commands: Vec<&'a str>) -> Constraints {
        Constraints { 
            permissions: permissions.iter().map(|p| p.to_string()).collect(),
            commands: external_commands.iter().map(|p| p.to_string()).collect()
        }
    }

    /** METHODS */

    pub fn permissions(&self) -> &Vec<String> {
        return &self.permissions;
    }

    pub fn throw_permission_exception<'a>(&self, scope: &mut v8::HandleScope<'a>, perm: &str) -> bool {
        if !self.has_permission(perm) {
            let e = v8::String::new(scope, format!("SecurityException -- Invalid permissions!\nYour extension does not have '{}'.", perm).as_str()).unwrap();
            let err = v8::Exception::error(scope, e);
            scope.throw_exception(err);
            return false;
        }
        return true;
    }   

    pub fn throw_command_exception<'a>(&self, scope : &mut v8::HandleScope<'a>, cmd: &str) -> bool {
        if !self.is_command_permitted(cmd) {
            let e = v8::String::new(scope, format!("SecurityException -- Invalid command declaration!\nYour extension has not declared the use of `{}`.", cmd).as_str()).unwrap();
            let err = v8::Exception::error(scope, e);
            scope.throw_exception(err);
            return false;
        }
        return true;
    }

    // Check to see if the  
    pub fn has_permission(&self, perm: &str) -> bool {
        return self.permissions.iter().map(|a| a.as_str()).map(Permission::new).any(|p| {
              p.has(perm)
        });
    }

    // Return a list of all possible commands the extension can run.
    pub fn commands(&self) -> &Vec<String> {
        return &self.commands;
    }

    // This check is ran
    //      when an extension is about to run a command
    //      to try to improve an extension's transparency.
    pub fn is_command_permitted(&self, command: &str) -> bool {
        return self.commands.contains(&command.to_string());
    }

    /** STATIC FUNCTIONS */

    pub fn into_scope<'a>(&self, scope: &mut v8::HandleScope<'a>) -> () {
        let global = scope.get_current_context().global(scope);

        let perms_unsafe = unsafe {
            mem::transmute::<&Constraints, *mut c_void>(&self)
        };

        let external_perms = v8::External::new(scope, perms_unsafe);

        let key = v8::String::new(scope, SECURITY_KEY).unwrap();
        global.define_own_property(scope, key.into(), external_perms.into(), v8::READ_ONLY);
    }

    pub fn from_scope<'a>(scope: &mut v8::HandleScope<'a>) -> &'a Constraints {
        let global = scope.get_current_context().global(scope);
        let s =  v8::String::new(scope, SECURITY_KEY).unwrap();
        let val = global.get(scope, s.into()).expect("Expected the ___SECURITY___ variable!");
        let v : v8::Local<v8::External> = unsafe {
            mem::transmute::<v8::Local<v8::Value>, v8::Local<v8::External>>(val)
        };

        unsafe {
            mem::transmute::<*mut c_void, &Constraints>(v.value())
        }
    }
}

