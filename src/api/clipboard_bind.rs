use v8;
use crate::api::{clipboard};

use super::utils_js;

// Validates arguments for all copy_* functions.
fn copy_validate<'a>(scope: &'a mut v8::HandleScope, _args: v8::FunctionCallbackArguments<'a>) -> Option<String> {
  if _args.length() > 0 {
    let err_str = v8::String::new(scope, "No text supplied!");
    v8::Exception::type_error(scope, err_str.unwrap());
    return None;
  }

  let str = _args.get(0);
   
  if !str.is_string() {
    let err_str = v8::String::new(scope, "Must supply a string!");
    v8::Exception::type_error(scope, err_str.unwrap());
    return None;
  }

  return str.to_rust_string_lossy(scope).into();
}

// Copy string to the clipboard.
pub fn copy_clipboard(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let str = copy_validate(scope, _args);
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);
  rv.set(prom.into());
  match str {
    None => 
      {prom.resolve(scope, undef.into()); },
    Some(x) => {
      clipboard::set_contents(x, clipboard::ClipSource::CLIPBOARD.into());
      prom.resolve(scope, undef.into());
    }
  }
}

// Copy string to the PRIMARY cipboard-source thing-y (select + middle click on Linux with X). 
pub fn copy_primary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let str = copy_validate(scope, _args);
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);
  rv.set(prom.into());
  match str {
    None => 
      {prom.resolve(scope, undef.into()); },
    Some(x) => {
      clipboard::set_contents(x, clipboard::ClipSource::PRIMARY.into());
      prom.resolve(scope, undef.into());
    }
  }
}

// Copy string to the SECONDARY cipboard-source thing-y (unused).
pub fn copy_secondary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let str = copy_validate(scope, _args);
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);
  rv.set(prom.into());

  match str {
    None => 
      {
        prom.resolve(scope, undef.into());
      },
    Some(x) => 
      {
        clipboard::set_contents(x, clipboard::ClipSource::SECONDARY.into());
        prom.resolve(scope, undef.into());
      }
  }
}

// TODO: Implement paste() function.

pub fn clear_clipboard(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);

  rv.set(prom.into());
  clipboard::set_contents("".into(), clipboard::ClipSource::CLIPBOARD.into());

  prom.resolve(scope, undef.into());
}

pub fn clear_primary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);

  rv.set(prom.into());
  clipboard::set_contents("".into(), clipboard::ClipSource::PRIMARY.into());

  prom.resolve(scope, undef.into());
}

pub fn clear_secondary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  let undef = v8::undefined(scope);

  rv.set(prom.into());
  clipboard::set_contents("".into(), clipboard::ClipSource::SECONDARY.into());

  prom.resolve(scope, undef.into());
}

pub fn read_validate<'a>(scope: &'a mut v8::HandleScope, _args: v8::FunctionCallbackArguments) -> Option<Vec<String>>  {
  if _args.length() == 0 {
    let err_str = v8::String::new(scope, "Must supply at least one type!");
    v8::Exception::type_error(scope, err_str.unwrap());
    return None;
  }

  let mut mime_types : Vec<String> = vec![];

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

pub fn read_js_arr<'a, 'b>(scope : &'a mut v8::HandleScope<'b>, res : Option<(String, String)>) -> v8::Local<'b, v8::Array> {
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

pub fn read_clipboard(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();

  let types = read_validate(scope, _args);
  rv.set(prom.into());

  if types.is_none() { return; }

  let res = clipboard::get_contents(
    clipboard::ClipSource::CLIPBOARD.into(), types.unwrap()
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn read_primary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();

  let types = read_validate(scope, _args);
  rv.set(prom.into());

  if types.is_none() { return; }

  let res = clipboard::get_contents(
    clipboard::ClipSource::PRIMARY.into(), types.unwrap()
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn read_secondary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();

  let types = read_validate(scope, _args);
  rv.set(prom.into());

  if types.is_none() { return; }

  let res = clipboard::get_contents(
    clipboard::ClipSource::SECONDARY.into(), types.unwrap()
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn read_text_clipboard(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  rv.set(prom.into());


  let res = clipboard::get_contents(
    clipboard::ClipSource::CLIPBOARD.into(), vec!["text/plain".to_string(), "UTF8_STRING".to_string()]
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn read_text_primary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();

  rv.set(prom.into());


  let res = clipboard::get_contents(
    clipboard::ClipSource::PRIMARY.into(), vec!["text/plain".to_string(), "UTF8_STRING".to_string()]
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn read_text_secondary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let prom = v8::PromiseResolver::new(scope).unwrap();

  rv.set(prom.into());


  let res = clipboard::get_contents(
    clipboard::ClipSource::SECONDARY.into(), vec!["text/plain".to_string(), "UTF8_STRING".to_string()]
  );

  let res_arr = read_js_arr(scope, res);
  prom.resolve(scope, res_arr.into());
}

pub fn formats_clipboard(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> ()  {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  rv.set(prom.into());

  let fs = clipboard::get_formats(clipboard::ClipSource::CLIPBOARD.into());

  let res = v8::Array::new(scope, fs.len() as i32);

  for (i, x) in fs.iter().enumerate() {
    let index = v8::Number::new(scope, i as f64);
    let str = v8::String::new(scope, x.as_str()).unwrap();
    res.set(scope, index.into(), str.into());
  }

  prom.resolve(scope, res.into());
}

pub fn formats_primary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> ()  {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  rv.set(prom.into());

  let fs = clipboard::get_formats(clipboard::ClipSource::PRIMARY.into());

  let res = v8::Array::new(scope, fs.len() as i32);

  for (i, x) in fs.iter().enumerate() {
    let index = v8::Number::new(scope, i as f64);
    let str = v8::String::new(scope, x.as_str()).unwrap();
    res.set(scope, index.into(), str.into());
  }

  prom.resolve(scope, res.into());
}

pub fn formats_secondary(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> ()  {
  let prom = v8::PromiseResolver::new(scope).unwrap();
  rv.set(prom.into());

  let fs = clipboard::get_formats(clipboard::ClipSource::SECONDARY.into());

  let res = v8::Array::new(scope, fs.len() as i32);

  for (i, x) in fs.iter().enumerate() {
    let index = v8::Number::new(scope, i as f64);
    let str = v8::String::new(scope, x.as_str()).unwrap();
    res.set(scope, index.into(), str.into());
  }

  prom.resolve(scope, res.into());
}
// xdotool has some sort of problem with keyboard layouts.
pub fn paste_text(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  
  if _args.length() == 0 || !_args.get(0).is_string() { 
    let err_str = v8::String::new(scope, "No text provided!").unwrap();
    v8::Exception::type_error(scope, err_str);
    return;
  }

  let contents = _args.get(0).to_rust_string_lossy(scope);

  let mut delay : u64 = 500;
  if _args.length() == 2 && _args.get(1).is_number() {
    delay = _args.get(1).integer_value(scope).unwrap_or(500) as u64;
  }

  let prom = v8::PromiseResolver::new(scope).unwrap();
  rv.set(prom.into());

  let res = clipboard::paste_text(contents, delay);

  let udef = v8::undefined(scope);

  if res {
    prom.resolve(scope, udef.into());
    return;
  }

  if _args.length() == 0 || !_args.get(0).is_string() { 
    let err_str = v8::String::new(scope, "Error pasting text!").unwrap();
    v8::Exception::type_error(scope, err_str);
    prom.resolve(scope, udef.into());
  }
}

pub fn source(scope: &mut v8::HandleScope, _args : v8::FunctionCallbackArguments, mut rv : v8::ReturnValue) -> () {
  let s = _args.get(0).to_rust_string_lossy(scope);

  match s.as_str() {
    "primary" => {
      let clipboard = v8::Object::new(scope);

      // Avdan.Clipboard.copy
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(copy_primary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "copy").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }
      // Avdan.Clipboard.paste 
      // There are numerous accuracy errors with this method due to xdotool -- sawry !
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(paste_text).build(scope).unwrap();
          let l = v8::String::new(scope, "paste").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.clear
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(clear_primary).build(scope).unwrap();
          let l = v8::String::new(scope, "clear").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.read
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_primary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "read").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.readText
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_text_primary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "readText").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.formats
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(formats_primary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "formats").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      rv.set(clipboard.into());
      return;
    },
    "secondary" => {
      let clipboard = v8::Object::new(scope);

      // Avdan.Clipboard.copy
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(copy_secondary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "copy").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }
      // Avdan.Clipboard.paste 
      // There are numerous accuracy errors with this method due to xdotool -- sawry !
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(paste_text).build(scope).unwrap();
          let l = v8::String::new(scope, "paste").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.clear
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(clear_secondary).build(scope).unwrap();
          let l = v8::String::new(scope, "clear").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.read
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_secondary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "read").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.readText
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_text_secondary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "readText").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.formats
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(formats_secondary).build(scope).unwrap();
          
          let n = v8::String::new(scope, "formats").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      rv.set(clipboard.into());
      return;
    },
    _ => {
      let clipboard = v8::Object::new(scope);

      // Avdan.Clipboard.copy
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(copy_clipboard).build(scope).unwrap();
          
          let n = v8::String::new(scope, "copy").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }
      // Avdan.Clipboard.paste 
      // There are numerous accuracy errors with this method due to xdotool -- sawry !
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(paste_text).build(scope).unwrap();
          let l = v8::String::new(scope, "paste").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.clear
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(clear_clipboard).build(scope).unwrap();
          let l = v8::String::new(scope, "clear").unwrap();
          utils_js::js_func_on_object(scope, &clipboard, l, f);
      }

      // Avdan.Clipboard.read
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_clipboard).build(scope).unwrap();
          
          let n = v8::String::new(scope, "read").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.readText
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(read_text_clipboard).build(scope).unwrap();
          
          let n = v8::String::new(scope, "readText").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      // Avdan.Clipboard.formats
      {
          let f = v8::FunctionBuilder::<v8::Function>::new(formats_clipboard).build(scope).unwrap();
          
          let n = v8::String::new(scope, "formats").unwrap();
          utils_js::js_func_on_object(
              scope,
              &clipboard,
              n,
              f
          );
      }

      rv.set(clipboard.into());
      return;
    }
  }
}


// Ideal solution, but rusty_v8 hates this (probably 'cause they know better)

// pub fn copy<'a>(source : clipboard::ClipSource) ->
//   impl Fn(&'a mut v8::HandleScope, v8::FunctionCallbackArguments<'a>, v8::ReturnValue<'a>) -> () {
//   return Box::new(|scope: &'a mut v8::HandleScope, _args : v8::FunctionCallbackArguments<'a>, mut rv : v8::ReturnValue<'a> | {
//     let str = copy_validate(scope, _args);
//     let prom = v8::PromiseResolver::new(scope).unwrap();
//     let undef = v8::undefined(scope);
//     rv.set(prom.into());
//     match str {
//       None => 
//         {prom.resolve(scope, undef.into()); },
//       Some(x) => {
//         clipboard::set_contents(x, clipboard::ClipSource::CLIPBOARD.into());
//         prom.resolve(scope, undef.into());
//       }
//     }
//   });
// }
