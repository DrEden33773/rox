use std::collections::HashMap;

use gc::{Finalize, Trace};

use crate::value::Value;

#[derive(Trace, Finalize, PartialEq, Eq)]
pub struct Class {
  pub member_value: HashMap<String, Value>,
}