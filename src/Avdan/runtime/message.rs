use std::any::TypeId;

use v8::{HandleScope, Local, Value};

use super::PromIndex;

type Out = Vec<u8>;
pub type Builder = for<'a> fn(&mut HandleScope<'a>, Out) -> Local<'a, Value>;

#[derive(Clone)]
pub enum Type {
    // Either Result or Error -- Causes Task to end.
    Result(Result<Out, String>, Builder),

    // Allows sending of event messages whilst task is in progress.
    Auxiliary(String,     Out,    Builder)
    //        Name ^ : Data ^ : Builder ^
}

impl Type {
    pub fn message(&self, prom_index: PromIndex) -> Message {
        Message (
            prom_index,
            self.to_owned()
        )
    }
}

pub struct Message (
    pub PromIndex,
    pub Type
);
