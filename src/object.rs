pub mod class;
pub mod str;

use self::class::Class;
use self::str::Str;
use gc::{Finalize, Gc, Trace};
use std::hash::Hash;

#[derive(Trace, Finalize, Debug, PartialEq, Eq, Hash)]
pub enum Object {
  String(Str),
  Class(Gc<Class>),
}
