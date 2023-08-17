use std::hash::Hash;

use super::Object;
use crate::value::Value;
use gc::{Finalize, Gc, Trace};

const SHORT_STR_MAX: usize = 14;
const MID_STR_MAX: usize = 48;

#[derive(Trace, Finalize, Debug)]
pub enum Str {
  ShortStr((u8, [u8; SHORT_STR_MAX])),
  MidStr(Gc<(u8, [u8; MID_STR_MAX])>),
  LongStr(Gc<Vec<u8>>),
}

impl Hash for Str {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let slice: &[u8] = self.into();
    slice.hash(state)
  }
}

impl std::ops::Add for Str {
  type Output = Str;
  fn add(self, rhs: Self) -> Self::Output {
    let mut temp = String::from(&self);
    temp += (&rhs).into();
    temp.into()
  }
}

impl From<&[u8]> for Str {
  fn from(seq: &[u8]) -> Self {
    match seq.len() {
      l if l <= SHORT_STR_MAX => {
        let mut buf = [0; SHORT_STR_MAX];
        buf[..l].copy_from_slice(seq);
        Self::ShortStr((l as u8, buf))
      }
      l if l <= MID_STR_MAX => {
        let mut buf = [0; MID_STR_MAX];
        buf[..l].copy_from_slice(seq);
        Self::MidStr(Gc::new((l as u8, buf)))
      }
      _ => Self::LongStr(Gc::new(seq.to_vec())),
    }
  }
}
impl From<Vec<u8>> for Str {
  fn from(vec: Vec<u8>) -> Self {
    vec.as_slice().into()
  }
}
impl From<&str> for Str {
  fn from(seq: &str) -> Self {
    seq.as_bytes().into()
  }
}
impl From<String> for Str {
  fn from(string: String) -> Self {
    string.into_bytes().into()
  }
}

impl<'a> From<&'a Str> for &'a [u8] {
  fn from(string: &'a Str) -> Self {
    match string {
      Str::ShortStr((len, buf)) => &buf[..*len as usize],
      Str::MidStr(str) => &str.1[..str.0 as usize],
      Str::LongStr(str) => str.as_slice(),
    }
  }
}
impl<'a> From<&'a Str> for &'a str {
  fn from(string: &'a Str) -> Self {
    std::str::from_utf8(string.into()).unwrap()
  }
}
impl From<&Str> for String {
  fn from(string: &Str) -> Self {
    String::from_utf8_lossy(string.into()).to_string()
  }
}

impl<'a> From<&'a Object> for &'a [u8] {
  fn from(obj: &'a Object) -> Self {
    match obj {
      Object::String(str) => str.into(),
      // simple serializer for class
      Object::Class(class) => Box::leak(Box::new(format!("{:#?}", class))).as_bytes(),
    }
  }
}
impl<'a> From<&'a Object> for &'a str {
  fn from(obj: &'a Object) -> Self {
    std::str::from_utf8(obj.into()).unwrap()
  }
}
impl From<&Object> for String {
  fn from(obj: &Object) -> Self {
    String::from_utf8_lossy(obj.into()).to_string()
  }
}

impl<'a> From<&'a Value> for &'a [u8] {
  fn from(value: &'a Value) -> Self {
    match value {
      Value::Nil => "nil".as_bytes(),
      Value::Boolean(b) => Box::leak(Box::new(b.to_string())).as_bytes(),
      Value::Integer(i) => Box::leak(Box::new(i.to_string())).as_bytes(),
      Value::Float(f) => Box::leak(Box::new(f.to_string())).as_bytes(),
      Value::Object(obj) => obj.into(),
    }
  }
}
impl<'a> From<&'a Value> for &'a str {
  fn from(value: &'a Value) -> Self {
    std::str::from_utf8(value.into()).unwrap()
  }
}
impl From<&Value> for String {
  fn from(value: &Value) -> Self {
    String::from_utf8_lossy(value.into()).to_string()
  }
}

impl PartialEq for Str {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::ShortStr(ls), Self::ShortStr(rs)) => ls.1[..ls.0 as usize] == rs.1[..rs.0 as usize],
      (Self::MidStr(ls), Self::MidStr(rs)) => ls.1[..ls.0 as usize] == rs.1[..rs.0 as usize],
      (Self::LongStr(ls), Self::LongStr(rs)) => *ls == *rs,
      _ => false,
    }
  }
}
impl Eq for Str {}
