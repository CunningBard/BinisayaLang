use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    ListRef(usize),

    ObjectRef(usize)
}

impl Value {
    pub fn is_object_ref(&self) -> bool {
        match self {
            Value::ObjectRef(_) => true,
            _ => false
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&String> {
        match self {
            Value::Str(value) => Some(value),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_list_ref(&self) -> Option<usize> {
        match self {
            Value::ListRef(list) => Some(*list),
            _ => None
        }
    }

    pub fn as_object_ref(&self) -> Option<&usize> {
        match self {
            Value::ObjectRef(object) => Some(object),
            _ => None
        }
    }
}
