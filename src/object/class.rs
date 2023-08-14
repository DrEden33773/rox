use crate::value::Value;
use gc::{Finalize, Trace};
use std::collections::HashMap;

#[derive(Trace, Finalize, PartialEq, Debug)]
pub struct Class {
  pub member_value: HashMap<String, Value>,
}
