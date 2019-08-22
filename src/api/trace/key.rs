use crate::api::registry;

pub struct Key {
    variable: registry::Variable,
}

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    String(String),
}

pub struct KeyValue {
    key: Key,
    value: Value,
}
