use std::{collections::HashMap, sync::mpsc::{Sender, Receiver, channel}};

use v8::{Global, Module, HandleScope, Local, Object};

use super::Specifier;

type Key = i32;
pub(crate) struct AvModStore {
    module_map: HashMap<Key, Sender<i32>>,
    modules   : HashMap<Specifier, Global<Module>>,
    internals : HashMap<Global<Module>, String>,
    json      : HashMap<Global<Module>, Global<Object>>,
}

impl AvModStore {
    pub fn into_scope<'a>(scope: &mut HandleScope<'a>) -> () {
        scope.set_slot(
            AvModStore {
                module_map: HashMap::new(),
                modules: HashMap::new(),
                internals: HashMap::new(),
                json: HashMap::new()
            }
        );
    }

    pub fn get_receiver<'a>(&mut self, script_id: i32, specifier: Specifier, module: Global<Module>) -> Option<Receiver<i32>> {
        let (tx, rx) = channel();
        self.module_map.insert(script_id, tx);
        Some(rx)
    }

    pub fn register<'a>(&mut self, specifier: Specifier, module: Global<Module>) -> () {
        if self.modules.contains_key(&specifier) {
            return;
        }
        self.modules.insert(specifier, module);
    }



    pub fn get(&self, specifier: &Specifier) -> Option<&Global<Module>> {
        self.modules.get(specifier)
    }

    pub fn get_sender(&self, id: &i32) -> Option<Sender<i32>> {
        match self.module_map.get(id) {
            Some(s,) => Some(s.clone()),
            None => None
        }
    }

    pub fn get_internal(&self, module: &Global<Module>) -> Option<&String> {
        self.internals.get(module)
    }

    pub fn add_internal(&mut self, module: Global<Module>, name: String) -> () {
        self.internals.insert(module, name);
    }

    pub fn add_json(&mut self, module: Global<Module>, json: Global<Object>) -> () {
        self.json.insert(module, json);
    }

    pub fn get_json(&mut self, module: &Global<Module>) -> Option<Global<Object>> {
        self.json.remove(module)
    }
}

