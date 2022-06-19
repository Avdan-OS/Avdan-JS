use std::collections::HashMap;
use super::{Body, Method};

struct Options {
    method  : Method,
    headers : HashMap<String, String>,
    body    : Body,
}

