use gc::{Finalize, Gc, Trace};

const SHORT_STR_MAX: usize = 14;
const MID_STR_MAX: usize = 48;

#[derive(Trace, Finalize)]
pub enum Str {
  ShortStr((u8, [u8; SHORT_STR_MAX])),
  MidStr(Gc<(u8, [u8; MID_STR_MAX])>),
  LongStr(Gc<Vec<u8>>),
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
  fn from(_str: &str) -> Self {
    _str.as_bytes().into()
  }
}
impl From<String> for Str {
  fn from(string: String) -> Self {
    string.into_bytes().into()
  }
}

impl PartialEq for Str {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::ShortStr(l0), Self::ShortStr(r0)) => l0.1[..l0.0 as usize] == r0.1[..r0.0 as usize],
      (Self::MidStr(l0), Self::MidStr(r0)) => l0.1[..l0.0 as usize] == r0.1[..r0.0 as usize],
      (Self::LongStr(l0), Self::LongStr(r0)) => *l0 == *r0,
      _ => false,
    }
  }
}
impl Eq for Str {}
