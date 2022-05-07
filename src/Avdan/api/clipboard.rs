use std::io::Write;
/**
 * Helper module for the Search API.
 *
 * A very simple wrapper around the `xclip` utility,
 * because trying to use X Rust bindings or rust X-Clipboard crates ended in absolute disaster.
 */
use std::process::{Command, Stdio};
use std::ptr;

use v8::{FunctionCallbackArguments, HandleScope, Object, Local};
use avdanos_search_macros::permission;
use crate::Avdan;
use crate::api::utils_js::{self, _Avdan as _Avdan};
use crate::core::JSApi;

pub struct AvClipboard {}

impl JSApi for AvClipboard {
    fn js<'s>(
        &self,
        scope: &mut v8::HandleScope<'s>
    ) -> Local<'s, Object> {
        let mut clipboard = v8::ObjectTemplate::new(scope);
        
        clipboard.set_internal_field_count(1);

        // <> Avdan.Clipboard

        ClipSource::assign_functions(scope, &mut clipboard);

        // Clipboard.source
        clipboard.set(
            utils_js::js_string(scope, "source").into(),
            v8::FunctionTemplate::builder(Self::source).build(scope).into(),
        );

        // </> Avdan.Clipboard
        let instance = clipboard.new_instance(scope).expect("No new instance! Ahhhh!");

        instance.set_internal_field(0, v8::String::new(scope, ClipSource::CLIPBOARD.name()).unwrap().into());

        return instance;
    }
}

impl AvClipboard {
    pub fn source(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) {
        if !args.get(0).is_string() {
            let except = _Avdan::Error::str("C-CLIP:NO-CLIP-FOUND", "Must be a valid clipboard source!").to_js(scope);
            scope.throw_exception(except.into());
            return;
        }

        let source = args.get(0).to_rust_string_lossy(scope);

        rv.set(match source.as_str() {
            "primary" => ClipSource::PRIMARY.js(scope),
            "secondary" => ClipSource::SECONDARY.js(scope),
            _ => ClipSource::CLIPBOARD.js(scope),
        }.into());
    }

    
}

#[derive(Copy, Clone)]
pub enum ClipSource {
    PRIMARY,
    SECONDARY,
    CLIPBOARD,
}

impl ClipSource {
    pub fn name(&self) -> &'static str {
        match self {
            ClipSource::PRIMARY => "primary",
            ClipSource::SECONDARY => "secondary",
            ClipSource::CLIPBOARD => "clipboard",
        }
    }

    pub fn get_formats(&self) -> Vec<String> {
        // Use xclip.
        let cmd = Command::new("xclip")
            .arg("-o")
            .arg("-selection")
            .arg(self.name())
            .arg("-t")
            .arg("TARGETS")
            .output();

        if cmd.is_err() {
            return vec![];
        }

        let cmd_out = String::from_utf8(cmd.unwrap().stdout).expect("Err converting to string!");

        // Split by newline characters.
        let lns = cmd_out.lines();

        let mut found = false;
        let mime_types: Vec<&str> = lns
            .map(|s| s)
            .filter(|&s| {
                if found {
                    return true;
                }
                if s.eq("MULTIPLE") {
                    found = true;
                }
                return false;
            })
            .collect();

        return mime_types
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<_>>();
    }

    pub fn get_contents(&self, types: Vec<String>) -> Option<(String, String)> {
        for t in types {
            let output = Command::new("xclip")
                .arg("-o")
                .arg("-selection")
                .arg(self.name())
                .arg("-t")
                .arg(t.clone())
                .output();

            if output.is_ok() {
                let out = String::from_utf8(output.unwrap().stdout).unwrap();
                if out.len() > 0 {
                    return (t, out).into();
                }
            }
        }

        return None;
    }

    pub fn get_raw_contents(&self, types: Vec<String>) -> Option<(String, Vec<u8>)> {
        for t in types {
            let output = Command::new("xclip")
                .arg("-o")
                .arg("-selection")
                .arg(self.name())
                .arg("-t")
                .arg(t.clone())
                .output()
                .unwrap();

            let out = output.stdout;
            if out.len() > 0 {
                return (t, out).into();
            }
        }

        return None;
    }

    pub fn set_contents(&self, contents: String) -> () {
        let mut cmd = Command::new("xclip")
            .arg("-i")
            .arg("-selection")
            .arg(self.name())
            .stdin(Stdio::piped())
            .spawn()
            .expect("Ahhhhhh!");

        let child_stdin = cmd.stdin.as_mut().unwrap();

        child_stdin
            .write_all(contents.as_bytes())
            .expect("Ahhh! Cannot write into stdin!");

        drop(cmd);
    }

    pub fn set_raw_contents(&self, contents: &[u8], mime_type: String) -> () {
        println!("Byte 0 and 1: {:x} {:x}", contents[0], contents[1]);

        let mut cmd = Command::new("xclip")
            .arg("-selection")
            .arg(self.name())
            .arg("-t")
            .arg(mime_type)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Ahhhhhh!");

        cmd.stdin
            .as_mut()
            .unwrap()
            .write_all(contents)
            .expect("Could not write into stdin!");

        // child_stdin.write_all_vectored(contents).expect("Ahhh! Cannot write into stdin!");

        drop(cmd);
    }

    // HACK: The below function is very buggy, and should be replaced with a different solution.
    pub fn paste_text(contents: String, delay: u64) -> bool {
        // Sleep for certain ms before typing.

        println!("Attempting to pase `{0}`", contents);

        std::thread::sleep(std::time::Duration::from_millis(delay));

        let cmd = Command::new("xdotool")
            .arg("getactivewindow")
            .arg("type")
            .arg(contents)
            .spawn();

        return cmd.is_ok();
    }

    pub fn from(str: String) -> Option<ClipSource> {
        match str.as_str() {
            "clipboard" => ClipSource::CLIPBOARD.into(),
            "primary" => ClipSource::PRIMARY.into(),
            "secondary" => ClipSource::SECONDARY.into(),
            _ => None,
        }
    }

    pub fn assign_functions<'s>(scope: &mut v8::HandleScope<'s>, obj: &mut v8::Local<'s, v8::ObjectTemplate>) -> () {
        // TODO: Use a macro to make the following less long...

        // Clipboard.copy
        obj.set(
            utils_js::js_string(scope, "copy").into(),
            v8::FunctionTemplate::builder(ClipboardJS::copy).build(scope).into(),
        );

        // Clipboard.copyRaw
        obj.set(
            utils_js::js_string(scope, "copyRaw").into(),
            v8::FunctionTemplate::builder(ClipboardJS::copy_raw).build(scope).into(),
        );
        
        // Clipboard.paste
        obj.set(
            utils_js::js_string(scope, "paste").into(),
            v8::FunctionTemplate::builder(ClipboardJS::paste_text).build(scope).into(),
        );
        
        // Clipboard.clear
        obj.set(
            utils_js::js_string(scope, "clear").into(),
            v8::FunctionTemplate::builder(ClipboardJS::clear).build(scope).into(),
        );
        
        // Clipboard.read
        obj.set(
            utils_js::js_string(scope, "read").into(),
            v8::FunctionTemplate::builder(ClipboardJS::read).build(scope).into(),
        );
        
        // Clipboard.readRaw
        obj.set(
            utils_js::js_string(scope, "readRaw").into(),
            v8::FunctionTemplate::builder(ClipboardJS::read_raw).build(scope).into(),
        );
        
        // Clipboard.readText
        obj.set(
            utils_js::js_string(scope, "readText").into(),
            v8::FunctionTemplate::builder(ClipboardJS::read_text).build(scope).into(),
        );
        
        // Clipboard.formats
        obj.set(
            utils_js::js_string(scope, "formats").into(),
            v8::FunctionTemplate::builder(ClipboardJS::formats).build(scope).into(),
        );
    }

    pub fn js<'s>(&self, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, Object> {
        let mut obj = v8::ObjectTemplate::new(scope);

        // Put an internal id for the clipboard source.
        obj.set_internal_field_count(1);

        Self::assign_functions(scope, &mut obj);

        let instance = obj.new_instance(scope).expect("No new instance! Ahhhh!");

        instance.set_internal_field(0, v8::String::new(scope, self.name()).unwrap().into());

        return instance;
    }
}
struct ClipboardJS {}

impl ClipboardJS {
    // Get the ClipSource from the parent object.
    fn get_source<'a, 'b>(
        scope: &'a mut HandleScope,
        args: &'b FunctionCallbackArguments,
    ) -> Option<ClipSource> {
        let source = args
            .this()
            .get_internal_field(scope, 0)
            .expect("Clipboard object should have this internal field!")
            .to_rust_string_lossy(scope);

        // println!("Calling for {}", source);
        ClipSource::from(source)
    }

    // Helper function for Clipboard.copy
    fn copy_validate<'a, 'b>(
        scope: &'a mut v8::HandleScope,
        args: &'b v8::FunctionCallbackArguments,
    ) -> Option<String> {
        if args.length() == 0 {
            let exception =
                _Avdan::Error::str("C-COPY-0000A", "String to copy is empty!").to_js(scope);
            scope.throw_exception(exception.into());
            return None;
        }

        let str = args.get(0);

        if !str.is_string() {
            let err_str = v8::String::new(scope, "Must supply a string!");
            let exception = v8::Exception::type_error(scope, err_str.unwrap());
            scope.throw_exception(exception);
            return None;
        }

        return str.to_rust_string_lossy(scope).into();
    }

    // Clipboard.copy
    #[permission(avdan.clipboard.write)]
    pub fn copy(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());
        let str = Self::copy_validate(scope, &args);

        if str.is_none() {
            let err =
                _Avdan::Error::str("C-COPY-0000", "Must provide a string to copy!").to_js(scope);
            prom.reject(scope, err);
            scope.throw_exception(err);
            return;
        }

        let contents = str.unwrap();
        match Self::get_source(scope, &args) {
            None => {
                let st = v8::String::new(scope, "Unknown clipboard source").unwrap();
                let err = v8::Exception::type_error(scope, st);
                prom.reject(scope, err);
                scope.throw_exception(err);
            }
            Some(source) => {
                source.set_contents(contents);
                let u = v8::undefined(scope).into();
                prom.resolve(scope, u);
            }
        }
    }

    // Clipboard.clear
    #[permission(avdan.clipboard.write)]
    pub fn clear(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());

        let source = Self::get_source(scope, &args).unwrap();

        source.set_contents("".to_string());

        let undef = v8::undefined(scope);

        prom.resolve(scope, undef.into());
    }

    /// Helper function for Clipboard.copyRaw
    fn check_args_raw_copy<'a>(args: &'a v8::FunctionCallbackArguments) -> bool {
        // TODO: Refactor into a single check.
        if args.length() < 2 {
            return false;
        }
        if !args.get(1).is_uint8_array() {
            return false;
        }
        if !args.get(0).is_string() {
            return false;
        }

        return true;
    }

    // Clipboard.copyRaw
    #[permission(avdan.clipboard.write)]
    pub fn copy_raw(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();

        println!("Checking arguments!");

        if !Self::check_args_raw_copy(&args) {
            let udef = v8::undefined(scope);
            prom.resolve(scope, udef.into());
            return;
        }
        // Get the MIME type of the data,
        let mime_type = args.get(0).to_string(scope).unwrap();

        // Get the raw data somehow...
        let content: v8::Local<v8::Uint8Array> = args.get(1).try_into().unwrap();

        let a = content.buffer(scope).unwrap();
        let i0 = a.get_index(scope, 0).unwrap();
        println!("Byte 0: {:x}", i0.int32_value(scope).unwrap());

        let store = content.buffer(scope).unwrap().get_backing_store();

        let mut bytes: Vec<u8> = Vec::with_capacity(content.byte_length());
        unsafe {
            bytes.set_len(content.byte_length());
        }
        let t = store.data().unwrap().as_ptr() as *const u8;
        unsafe {
            ptr::copy_nonoverlapping(
                t.as_ref().unwrap(),
                bytes.as_mut_ptr(),
                content.byte_length(),
            )
        }

        println!("Bytes 0 and 1: {:x} {:x}", &bytes[0], &bytes[1]);

        let source = Self::get_source(scope, &args).unwrap();

        source.set_raw_contents(&bytes, mime_type.to_rust_string_lossy(scope));
    }

    // Helper functions for Clipboard.read

    fn read_validate(
        scope: &mut v8::HandleScope,
        _args: &v8::FunctionCallbackArguments,
    ) -> Option<Vec<String>> {
        if _args.length() == 0 {
            let err_str = v8::String::new(scope, "Must supply at least one type!");
            let e = v8::Exception::type_error(scope, err_str.unwrap());
            scope.throw_exception(e);
            return None;
        }

        let mut mime_types: Vec<String> = vec![];

        // Validate each argument
        for i in 0..(_args.length()) {
            if !_args.get(i).is_string() {
                let err_str = v8::String::new(scope, "Types must be strings!");
                v8::Exception::type_error(scope, err_str.unwrap());
                return None;
            }
            mime_types.push(_args.get(i).to_rust_string_lossy(scope));
        }

        return mime_types.into();
    }

    fn read_js_arr<'a>(
        scope: &mut v8::HandleScope<'a>,
        res: Option<(String, String)>,
    ) -> v8::Local<'a, v8::Array> {
        let res_arr = v8::Array::new(scope, 2);

        let index_0 = v8::Number::new(scope, 0f64);
        let index_1 = v8::Number::new(scope, 1f64);

        if res.is_none() {
            let undef = v8::undefined(scope);
            res_arr.set(scope, index_0.into(), undef.into());
            res_arr.set(scope, index_1.into(), undef.into());
            return res_arr;
        }

        let type_js = v8::String::new(scope, res.clone().unwrap().0.as_str()).unwrap();
        let content_js = v8::String::new(scope, res.clone().unwrap().1.as_str()).unwrap();

        res_arr.set(scope, index_0.into(), type_js.into());
        res_arr.set(scope, index_1.into(), content_js.into());

        return res_arr;
    }

    // Clipboard.readRaw
    #[permission(avdan.clipboard.read)]
    pub fn read_raw(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();

        let types = Self::read_validate(scope, &args);
        rv.set(prom.into());

        if types.is_none() {
            return;
        }

        let source = Self::get_source(scope, &args).unwrap();

        let res = source.get_raw_contents(types.unwrap());

        let res_tuple = v8::Array::new(scope, 2);

        if res == None {
            let undef = v8::undefined(scope);
            res_tuple.set_index(scope, 0, undef.into());
            res_tuple.set_index(scope, 1, undef.into());
            prom.resolve(scope, res_tuple.into());
            return;
        }

        let tmp_tuple = res.unwrap();
        let mime = v8::String::new(scope, tmp_tuple.0.as_str()).unwrap();
        res_tuple.set_index(scope, 0, mime.into());

        // Attempt to send the raw binary to JS as a Uint8Array

        let data = tmp_tuple.1;
        let size: usize = data.len();
        let res_bytes = data.into_boxed_slice();
        let res_js_byte_store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(res_bytes);
        let res_js_byte_buf =
            v8::ArrayBuffer::with_backing_store(scope, &res_js_byte_store.make_shared());
        let res_js_bytes = v8::Uint8Array::new(scope, res_js_byte_buf, 0, size).unwrap();

        res_tuple.set_index(scope, 1, res_js_bytes.into());
        prom.resolve(scope, res_tuple.into());
    }

    // Clipboard.read
    #[permission(avdan.clipboard.read)]
    pub fn read(
        scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();

        let types = Self::read_validate(scope, &args);
        rv.set(prom.into());

        if types.is_none() {
            return;
        }

        let source = Self::get_source(scope, &args).unwrap();

        let res = source.get_contents(types.unwrap());

        let res_arr = Self::read_js_arr(scope, res);

        prom.resolve(scope, res_arr.into());
    }

    // Clipboard.readText
    #[permission(avdan.clipboard.read)]
    pub fn read_text(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> () {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());
      
        let source = Self::get_source(scope, &args).unwrap();
        let res = source.get_contents(vec!["text/plain".to_string(), "UTF8_STRING".to_string()]);
        let res_arr = Self::read_js_arr(scope, res);
        let val = res_arr.get_index(scope, 1).unwrap().into();
        prom.resolve(scope, val);
      }

    // Clipboard.formats
    #[permission(avdan.clipboard.read)]
    pub fn formats(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> ()  {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());
        
        let source = Self::get_source(scope, &args).unwrap();

        let fs = source.get_formats();
      
        let res = v8::Array::new(scope, fs.len() as i32);
      
        for (i, x) in fs.iter().enumerate() {
          let index = v8::Number::new(scope, i as f64);
          let str = v8::String::new(scope, x.as_str()).unwrap();
          res.set(scope, index.into(), str.into());
        }
      
        prom.resolve(scope, res.into());
      }

    // Clipboard.paste
    #[permission(avdan.clipboard.type)]
    pub fn paste_text(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) -> () {
        if args.length() == 0 || !args.get(0).is_string() { 
          let err_str = v8::String::new(scope, "No text provided!").unwrap();
          v8::Exception::type_error(scope, err_str);
          return;
        }
      
        let contents = args.get(0).to_rust_string_lossy(scope);
      
        let mut delay : u64 = 500;
        if args.length() == 2 && args.get(1).is_number() {
          delay = args.get(1).integer_value(scope).unwrap_or(500) as u64;
        }
      
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());
      
        let res = ClipSource::paste_text(contents, delay);
      
        let udef = v8::undefined(scope);
      
        if res {
          prom.resolve(scope, udef.into());
          return;
        }
      
        if args.length() == 0 || !args.get(0).is_string() { 
          let err_str = v8::String::new(scope, "Error pasting text!").unwrap();
          v8::Exception::type_error(scope, err_str);
          prom.resolve(scope, udef.into());
        }
      }
}
