use std::{fs::{File, self}, io::{Write, Read}, slice};

use v8::{Local, HandleScope, Value, ObjectTemplate, Object};

use super::utils_js::{self, Avdan};

pub struct AvFile {}

enum IOError {
    OSError(i32),
    GenericError(i32, String),
}

impl IOError {
    pub fn JS<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Value> {
        match self {
            Self::OSError(id) =>
                utils_js::Avdan::Error::new(
                    format!("F-OS{:?}", id),
                    format!(
                        "Error from OS with code: {:?}\nSee https://mariadb.com/kb/en/operating-system-error-codes/ for more information.",
                        id
                    )
                ).to_js(scope),
            Self::GenericError(code, msg) => 
                utils_js::Avdan::Error::new(format!("{}", code), msg.to_string()).to_js(scope)
        }
        
    }
}

impl AvFile {

    pub fn new() -> AvFile {
        AvFile {}
    }

    // Avdan.File.write
    fn write(path: String, to_write : &[u8]) -> Result<(), IOError> {
        let file = File::create(path);

        match file {
            Err(err) => Err(IOError::OSError(err.raw_os_error().unwrap_or(-1))),
            Ok(mut f) => 
                match f.write_all(&to_write) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(IOError::GenericError(3, "Generic IO Error!".to_string()))
                }
            
        }
    }

    fn read(path: String) -> Result<Vec<u8>, IOError> {
        match fs::read(path) {
            Err(err) => Err(IOError::OSError(err.raw_os_error().unwrap_or(-1))),
            Ok(f) => {
                println!("Found {} bytes.", f.len());
                return Ok(f)
            },
        }
    }


    pub fn js<'s>(&self, scope: &mut v8::HandleScope<'s>) -> Local<'s, Object> {
        let mut file_api = ObjectTemplate::new(scope);

        // Avdan.File <>
        FileJS::assign_functions(scope, &mut file_api);

        // Avdan.File </>

        file_api.new_instance(scope).unwrap()
    }
}

enum ReadType {
    BINARY,
    UTF8
}
struct FileJS {}

impl FileJS {

    fn assign_functions<'s>(
        scope: &mut v8::HandleScope<'s>,
        obj: &mut v8::Local<'s, v8::ObjectTemplate>
    ) -> () {
        // File.write
        obj.set(
            utils_js::js_string(scope, "write").into(),
            v8::FunctionTemplate::builder(Self::write).build(scope).into(),
        );

        // File.read
        obj.set(
            utils_js::js_string(scope, "read").into(),
            v8::FunctionTemplate::builder(Self::read).build(scope).into(),
        );

    }

    fn check_path_arg<'a, 'b>(
        scope: &mut v8::HandleScope<'a>,
        args : &'b v8::FunctionCallbackArguments,
    ) -> Result<String, IOError> {
        if !args.get(0).is_string() {
            return Err(IOError::GenericError(0, "Path must be a string".to_string()));
        }
        return Ok(args.get(0).to_rust_string_lossy(scope));
    }

    pub fn write(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());

        match Self::check_path_arg(scope, &args) {
            Ok(_) => {},
            Err(e) => {
                let excep = e.JS(scope);
                prom.resolve(scope, excep);
                scope.throw_exception(excep);
                return;
            }
        }

        let data : Vec<u8>;
        if args.get(1).is_string() {
            // Convert string to raw bytes
            let content : v8::Local<v8::String> = args.get(1).try_into().unwrap();
            let str = content.to_rust_string_lossy(scope);
            data = str.as_bytes().to_vec();

        } else if args.get(1).is_uint8_array() {
            // We are "raw" bytes 
            let content : v8::Local<v8::Uint8Array> = args.get(1).try_into().unwrap();

            let buf = content.buffer(scope).unwrap();

            let store = buf.get_backing_store();
            let len = store.byte_length();

            let t =store.data().unwrap().as_ptr() as *const u8;
            data = unsafe {
                slice::from_raw_parts(t, len)
            }.to_vec();
        } else {
            let err = Avdan::Error::str("F001", "Invalid content type! Must be either Uint8Array or string!").to_js(scope);
            scope.throw_exception(err);
            prom.reject(scope, err);
            return;
        }

        match AvFile::write(args.get(0).to_rust_string_lossy(scope), &data) {
            Ok(_) => {
                let udef = v8::undefined(scope);
                prom.resolve(scope, udef.into());
            },
            Err(e) => {
                let err = e.JS(scope);
                scope.throw_exception(err);
                prom.reject(scope, err);
            }
        }
    }


    

    fn check_type_arg<'a, 'b>(
        scope: &mut v8::HandleScope<'a>,
        args : &'b v8::FunctionCallbackArguments,
    ) -> Result<ReadType, IOError> {
        if !args.get(1).is_string() {
            return Err(IOError::GenericError(1100, "Format argument not present!".to_string()));
        }
        match args.get(1).to_rust_string_lossy(scope).as_str() {
            "utf8" => Ok(ReadType::UTF8),
            "bytes"  => Ok(ReadType::BINARY),
            _ => Err(IOError::GenericError(1101, "Format argument must be 'utf8' or 'bytes'!".to_string()))
        }
    }

    pub fn read(
        scope: &mut v8::HandleScope,
        args : v8::FunctionCallbackArguments,
        mut rv : v8::ReturnValue
    ) {
        let prom = v8::PromiseResolver::new(scope).unwrap();
        rv.set(prom.into());
        match Self::check_path_arg(scope, &args) {
            Err(e) => {
                let except = e.JS(scope);
                scope.throw_exception(except);
                prom.reject(scope, except);
                return
            },
            Ok(path) => {
                match  Self::check_type_arg(scope, &args) {
                    Err(e) => {
                        let except = e.JS(scope);
                        scope.throw_exception(except);
                        prom.reject(scope, except);
                        return;
                    },
                    Ok(t) => {
                        let file_contents = AvFile::read(path);
                        match file_contents {
                            Err(e) => {
                                let err = e.JS(scope);
                                scope.throw_exception(err);
                                prom.reject(scope, err);
                                return;
                            },
                            Ok(content) => {
                                match t {
                                    ReadType::BINARY => {
                                        let len = content.len();
                                        // Convert bytes into a Uint8Array
                                        let js_byte_store = v8::ArrayBuffer::new_backing_store_from_boxed_slice(content.into_boxed_slice());
                                        let js_bytes =
                                            v8::ArrayBuffer::with_backing_store(scope, &js_byte_store.make_shared());
                                        let js_byte_arr = v8::Uint8Array::new(scope, js_bytes, 0, len).unwrap();

                                        prom.resolve(scope, js_byte_arr.into());
                                    },
                                    ReadType::UTF8 => {
                                        // Convert bytes to utf8 string.
                                        let str = String::from_utf8(content);
                                        match str {
                                            Err(_) => {
                                                let err = Avdan::Error::str("F-1200", "Error converting bytes to string!").to_js(scope);
                                                scope.throw_exception(err);
                                                prom.reject(scope, err);
                                                return;
                                            },
                                            Ok(s) => {
                                                let js_str = v8::String::new(scope, s.as_str()).unwrap();
                                                prom.resolve(scope, js_str.into());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}