use std::time::Duration;

use crate::{
    core::{def_safe_property, JSApi}, Avdan::{runtime::{task::output, Task, Type}},
};
use colored::*;
use v8::{
    FunctionCallbackArguments, HandleScope, Local, Object,
    ReturnValue, Value, Number,
};

use crate::core::def_safe_function;

enum Colors {
    String(String),
    Symbol(String),
    Number(String),
    Special(String),
    Error(String),
    BracketMatch(u8, String),
}

impl Colors {
    fn to_string(&self) -> String {
        match *self {
            Colors::String(ref s) => s.truecolor(39, 154, 241).to_string(), // Light-ish Blue
            // Colors::SYMBOL(ref s) => s.truecolor(20, 129, 186).to_string(),
            Colors::Symbol(ref s) => s.truecolor(242, 67, 51).to_string(),
            Colors::Number(ref s) => s.truecolor(254, 153, 32).to_string(),
            Colors::Special(ref s) => s.truecolor(105, 72, 115).to_string(),
            Colors::Error(ref s) => s.truecolor(239, 91, 91).to_string(),
            Colors::BracketMatch(ref lvl, ref s) => {
                let level_colors: Vec<(u8, u8, u8)> =
                    vec![(151, 223, 252), (127, 176, 105), (115, 29, 216)];
                let (r, g, b) = level_colors[(lvl % 3) as usize];
                s.truecolor(r, g, b).to_string()
            }
        }
    }
}

pub struct AvDebug {}

impl JSApi for AvDebug {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        let obj = v8::Object::new(scope);
        def_safe_function!(scope, obj, "log", AvDebug::log);
        def_safe_function!(scope, obj, "wait", AvDebug::wait);
        
        obj
    }
}

impl AvDebug {
    fn type_of(value: v8::Local<v8::Value>) -> &str {
        match value {
            v if v.is_string() => "string",
            v if v.is_promise() => "promise",
            v if v.is_function() => "function",
            v if v.is_number() => "number",
            v if v.is_array() => "array",
            v if v.is_symbol() => "symbol",
            v if v.is_object() => "object",
            v if v.is_uint8_array() => "uint8_array",

            _ => "unknown",
        }
    }

    // Debug.log
    pub fn log(
        scope: &mut HandleScope,
        args: v8::FunctionCallbackArguments,
        rv: v8::ReturnValue,
    ) -> () {
        let mut out: Vec<String> = vec![];
        for i in 0..args.length() {
            out.push(Self::inspect(scope, args.get(i), Some(0)));
        }

        println!("{}", out.join(" "));
    }

    pub fn tmp_helper_wait_tick<'a>(scope: &mut HandleScope<'a>, vec: Vec<u8>) -> Local<'a, Value> {
        let _tick = vec
            .get(0)
            .expect("Avdan.Debug.wait bad auxiliary Object Stricture !")
            .to_owned();

        let obj = Object::new(scope);
        let tick = Number::new(scope, _tick as f64);
        def_safe_property(scope, obj, "tick", tick.into());

        obj.into()
    }

    // Debug.log
    pub fn wait (
        scope  : &mut HandleScope,
        args   : FunctionCallbackArguments,
        mut rv : ReturnValue,
    ) -> () {
        let ticks = args.get(0).int32_value(scope).unwrap_or(10);
        let ms = args.get(1).int32_value(scope).unwrap_or(1000);

        println!("Starting timeout!");

        let prom = Task::new (
            scope, 
            move |(id, tx)| {
                for tick in 0..ticks {
                    std::thread::sleep(Duration::from_millis(ms.try_into().unwrap()));
                    tx.send (
                        Type::Auxiliary(
                            "tick".to_string(),
                            vec![tick as u8],
                            Self::tmp_helper_wait_tick,
                        )
                        .message(id),
                    )
                    .expect("Error sending to runtime!");
                }
                Ok(vec![])
            },
            
            output::void
        );

        rv.set(prom.into());
    }

    // pub fn fetch(
    //     scope: &mut HandleScope,
    //     args: FunctionCallbackArguments,
    //     mut rv: ReturnValue,
    // ) -> () {
    //     let url = args.get(0).to_rust_string_lossy(scope);

    //     let prom = Task::new(
    //         scope,
    //         move |_tx| {
    //             let req = reqwest::blocking::get(url);
    //             match req {
    //                 Err(err) => Err(err.to_string()),
    //                 Ok(res) => match res.bytes() {
    //                     Ok(b) => Ok(b.to_vec()),
    //                     Err(err) => Err(err.to_string()),
    //                 },
    //             }
    //         },
    //         output::uint8_array
    //     );

    //     rv.set(prom.into());
    // }

    // Simple inspector <Not Complete>
    pub fn inspect(scope: &mut HandleScope, value: Local<Value>, level: Option<u8>) -> String {
        let lvl = level.unwrap_or(0);
        
        match Self::type_of(value) {
            "promise" => Colors::Special("Promise".to_string()).to_string(), 
            "uint8_array" => {
                Colors::BracketMatch(lvl, Self::inspect_uint8_array(scope, value, lvl)).to_string()
            }
            "array" => Colors::BracketMatch(
                lvl,
                format!("[ {} ]", Self::inspect_array(scope, value, lvl)),
            )
            .to_string(),
            "function" => Colors::Special(Self::inspect_function(scope, value)).to_string(),
            "string" => Self::inspect_string(scope, value.into()),
            "number" => {
                Colors::Number(format!("{}", value.to_rust_string_lossy(scope))).to_string()
            }
            "symbol" => Colors::Symbol(Self::inspect_symbol(scope, value)).to_string(),
            "object" => Colors::BracketMatch (
                lvl,
                format! (
                    "{{\n{1}\n{0}}}",
                    str::repeat("   ", lvl as usize),
                    Self::inspect_object(scope, value, lvl)
                ),
            )
            .to_string(),
            _ => Colors::Special(format!("{}", value.to_rust_string_lossy(scope))).to_string(),
        }
    }

    // Helper inspector functions.
    fn inspect_string(scope: &mut HandleScope, str: Local<Value>) -> String {
        let raw = str.to_rust_string_lossy(scope);
        let list: Vec<String> = raw
            .split("\n")
            .map(|s| Colors::String(format!("\"{}\"", s)).to_string())
            .collect();
        
        return list.join("\n  + ");
    }

    fn inspect_array(scope: &mut HandleScope, arr: Local<Value>, lvl: u8) -> String {
        let p = &*arr;

        let array: &v8::Array = unsafe {
            std::mem::transmute::<*const Value, *const v8::Array>(p)
                .as_ref()
                .unwrap()
        };

        // Get the first 32 items.
        let mut items: Vec<String> = vec![];
        
        for i in 0..(if array.length() > 32 {
            32
        } else {
            array.length()
        }) {
            let index = v8::Number::new(scope, i as f64);
            let el = array.get(scope, index.into()).unwrap();
            items.push(Self::inspect(scope, el, Some(lvl + 1)));
        }
        return format! (
            "{0}{1}",
            items.join(", "),
            (if array.length() > 32 {
                format!(",\nAnd {} more...\n", array.length() - 32)
            } else {
                String::from("")
            })
        );
    }

    pub(crate) fn inspect_object(scope: &mut HandleScope, object: Local<Value>, lvl: u8) -> String {
        let p = &*object;

        let obj: &v8::Object = unsafe {
            std::mem::transmute::<*const Value, *const v8::Object>(p)
                .as_ref()
                .unwrap()
        };

        let props = obj.get_own_property_names(scope).unwrap();
        let mut out: Vec<String> = vec![];
        
        for i in 0..props.length() {
            let index = v8::Number::new(scope, i as f64);
            let prop = props.get(scope, index.into()).unwrap();
            let name = Self::inspect(scope, prop, Some(lvl + 1));
            let value = obj.get(scope, prop.into()).unwrap();
            let val = Self::inspect(scope, value, Some(lvl + 1));
            out.push(format!(
                "{0}{1}: {2}",
                str::repeat("   ", (lvl + 1) as usize),
                name,
                val
            ));
        }
        
        return format!("{0}", out.join(", \n"));
    }

    fn inspect_function(scope: &mut HandleScope, function: Local<Value>) -> String {
        let p = &*function;

        // Unsafe cast to v8::Function as Rust's playing too safe.
        // Unless there's a safer way to cast v8::Value -> v8::Function, which could exist.
        let func: &v8::Function = unsafe {
            std::mem::transmute::<*const Value, *const v8::Function>(p)
                .as_ref()
                .unwrap()
        };

        return match func.get_name(scope).to_rust_string_lossy(scope).as_str() {
            "" => "[Function (anonymous)]".to_string(),
            n => format!("[Function: {}]", n).to_string(),
        };
    }

    fn inspect_symbol(scope: &mut HandleScope, symbol: Local<Value>) -> String {
        let p = &*symbol;

        // Unsafe cast to v8::Symbol as Rust's playing too safe.
        // Unless there's a safer way to cast v8::Value -> v8::Function, which could exist.
        let sym: &v8::Symbol = unsafe {
            std::mem::transmute::<*const Value, *const v8::Symbol>(p)
                .as_ref()
                .unwrap()
        };

        let d = sym.description(scope);

        return match sym.description(scope).is_undefined() {
            true => String::from("Symbol()"),
            false => String::from(format!("Symbol({})", Self::inspect(scope, d, Some(0)))),
        };
    }

    fn inspect_uint8_array(scope: &mut HandleScope, uint8_arr: Local<Value>, lvl: u8) -> String {
        let p = &*uint8_arr;

        let arr : &v8::Uint8Array = unsafe {
            std::mem::transmute::<*const Value, *const v8::Uint8Array>(p)
                .as_ref()
                .unwrap()
        };
        
        return format! (
            "{}({})",
            Colors::Special("Uint8Array".to_string()).to_string(),
            arr.byte_length()
        );
    }
}
