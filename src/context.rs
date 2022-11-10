use std::collections::HashMap;
use std::ops;

use crate::intern::*;
use crate::object::*;
use crate::value::*;

pub struct Context {
    map: HashMap<IString, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, idx: IString) -> Option<&Value> {
        self.map.get(&idx)
    }

    pub fn get_mut(&mut self, idx: IString) -> Option<&mut Value> {
        self.map.get_mut(&idx)
    }

    #[track_caller]
    pub fn get_or_insert(
        &mut self,
        idx: IString,
        val: Value,
    ) -> &mut Value {
        self.map.entry(idx).or_insert(val)
    }

    #[track_caller]
    pub fn get_or_insert_with(
        &mut self,
        idx: IString,
        func: impl FnOnce() -> Value,
    ) -> &mut Value {
        self.map.entry(idx).or_insert_with(func)
    }
}

impl ops::Index<IString> for Context {
    type Output = Value;

    #[track_caller]
    fn index(&self, idx: IString) -> &Value {
        self.get(idx)
            .expect("context contains no value for name {idx}")
    }
}

impl ops::IndexMut<IString> for Context {
    #[track_caller]
    fn index_mut(&mut self, idx: IString) -> &mut Value {
        self.get_or_insert(idx, Value::Object(Object::Null))
    }
}
