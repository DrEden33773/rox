use std::hash::Hash;

use gc::{Finalize, Trace};

use crate::object::Object;

#[derive(Trace, Finalize, Debug)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  Object(Object),
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Nil, Value::Nil) => true,
      (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
      (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
      (Self::Float(l0), Self::Float(r0)) => l0 == r0,
      (Self::Object(l0), Self::Object(r0)) => l0 == r0,
      _ => false,
    }
  }
}

impl Eq for Value {}

impl Hash for Value {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Value::Nil => panic!("unsupported key type: nil"),
      Value::Boolean(b) => b.hash(state),
      Value::Integer(i) => i.hash(state),
      Value::Float(f) => {
        if f.is_nan() {
          panic!("unsupported key: NaN")
        } else {
          f.to_bits().hash(state)
        }
      }
      Value::Object(obj) => obj.hash(state),
    }
  }
}
