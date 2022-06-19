use v8::{HandleScope, Local, Object};

use crate::core::JSApi;

/*
    A collection of network functions.
    Should contain Fetch API
*/

enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

// Default to GET if name not valid.

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "head" => Self::HEAD,
            "post" => Self::POST,
            "put" => Self::PUT,
            "delete" => Self::DELETE,
            "connect" => Self::CONNECT,
            "options" => Self::OPTIONS,
            "trace" => Self::TRACE,
            "patch" => Self::PATCH,
            _ => Self::GET,
        }
    }
}

enum Body {
    Bytes(Vec<u8>),
    Text(String),
    // Todo stream, or pipe
}

// Simplified version of the fetch API
mod fetch;

struct Net {

}

impl JSApi for Net {
    fn js<'a>(&self, scope: &mut HandleScope<'a>) -> Local<'a, Object> {
        todo!();
    }
}