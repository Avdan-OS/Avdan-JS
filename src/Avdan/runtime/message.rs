use std::any::TypeId;

use v8::{HandleScope, Local, Value};

use super::PromIndex;


type Builder = for<'a> fn(&mut HandleScope<'a>, Vec<u8>) -> Local<'a, Value>;

#[derive(Clone)]
pub enum Type {
    // Either Result or Error -- Causes Task to end.
    Result(Result<Vec<u8>, String>, TypeId),

    // Reserved for later use -- Allows sending of event messages whilst task is in progress.
    Auxiliary(String, Vec<u8>, Builder)
}

impl Type {
    pub fn message(&self, prom_index: PromIndex) -> Message {
        Message(
            prom_index,
            self.to_owned()
        )
    }
}

pub struct Message(
    pub PromIndex,
    pub Type
);