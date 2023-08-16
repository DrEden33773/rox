use crate::value::Value;
use gc::{Finalize, Trace};
use std::{collections::HashMap, hash::Hash};

#[derive(Trace, Finalize, PartialEq, Eq, Debug)]
pub struct Class {
  pub field_value: HashMap<Value, Value>,
}

impl Hash for Class {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.field_value.keys().for_each(|key| key.hash(state))
  }
}
