use std::intrinsics::transmute;

use v8::{FixedArray, Local, HandleScope, Data};

pub fn fixed_array_to_vec<'a, R>(scope: &mut HandleScope<'a>, array: Local<FixedArray>) -> Vec<Local<'a, R>> {
    let mut out : Vec<Local<R>> = vec![];
    for i in 0..array.length() {
        out.push(
            unsafe {
                transmute::<Local<Data>, Local<R>>(array.get(scope, i).unwrap())
            }
        )
    }
    out
}