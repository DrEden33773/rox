pub mod class;
pub mod str;

use gc::{Finalize, Gc, Trace};

use self::class::Class;
use self::str::Str;

#[derive(Trace, Finalize)]
pub enum Object {
  String(Str),
  Class(Gc<Class>),
}

impl PartialEq for Object {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::String(l0), Self::String(r0)) => l0 == r0,
      (Self::Class(l0), Self::Class(r0)) => *l0 == *r0,
      _ => false,
    }
  }
}
