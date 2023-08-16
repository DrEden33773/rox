use std::{
  fmt::{Debug, Display},
  ptr::NonNull,
};

pub mod iter;

#[derive(Default)]
pub struct ListNode<T: Default> {
  prev: Option<NonNull<ListNode<T>>>,
  next: Option<NonNull<ListNode<T>>>,
  value: T,
}

impl<T: Default> ListNode<T> {
  pub fn new(value: T) -> Self {
    Self {
      prev: None,
      next: None,
      value,
    }
  }
}

impl<T: Default + Display> Display for ListNode<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl<T: Default + Debug> Debug for ListNode<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ListNode")
      .field("prev", &self.prev)
      .field("next", &self.next)
      .field("value", &self.value)
      .finish()
  }
}

impl<T: Default + PartialEq> PartialEq for ListNode<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}

impl<T: Default + Eq> Eq for ListNode<T> {}

pub struct LinkedList<T: Default> {
  head: NonNull<ListNode<T>>,
  tail: NonNull<ListNode<T>>,
  len: usize,
}

impl<T: Default + Display> Display for LinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, e) in self.iter().enumerate() {
      write!(f, "{}", e)?;
      if i != self.len - 1 {
        write!(f, ", ")?;
      }
    }
    write!(f, "]")?;
    Ok(())
  }
}

impl<T: Default + Debug> Debug for LinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("LinkedList")
      .field("head", &self.head)
      .field("tail", &self.tail)
      .field("len", &self.len)
      .finish()
  }
}

impl<T: Default + Clone> Clone for LinkedList<T> {
  fn clone(&self) -> Self {
    self.iter().cloned().collect::<Self>()
  }
}

impl<T: Default + PartialEq> PartialEq for LinkedList<T> {
  fn eq(&self, other: &Self) -> bool {
    for (a, b) in self.iter().zip(other.iter()) {
      if a != b {
        return false;
      }
    }
    true
  }
}

impl<T: Default + Eq> Eq for LinkedList<T> {}

impl<T: Default> Default for LinkedList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Default> LinkedList<T> {
  pub fn new() -> Self {
    let mut head = Box::<ListNode<T>>::default();
    let mut tail = Box::<ListNode<T>>::default();
    head.next = Some(unsafe { NonNull::new_unchecked(tail.as_mut()) });
    tail.prev = Some(unsafe { NonNull::new_unchecked(head.as_mut()) });

    Self {
      head: unsafe { NonNull::new_unchecked(Box::leak(head)) },
      tail: unsafe { NonNull::new_unchecked(Box::leak(tail)) },
      len: 0,
    }
  }

  pub fn push_front(&mut self, value: T) {
    let new_node = Box::new(ListNode::new(value));

    let mut new = unsafe { NonNull::new_unchecked(Box::leak(new_node)) };
    let mut head_next = unsafe { self.head.as_mut() }.next.unwrap();

    unsafe {
      new.as_mut().prev = Some(self.head);
      self.head.as_mut().next = Some(new);
      new.as_mut().next = Some(head_next);
      head_next.as_mut().prev = Some(new);
    };

    self.len += 1;
  }

  pub fn push_back(&mut self, value: T) {
    let new_node = Box::new(ListNode::new(value));

    let mut new = unsafe { NonNull::new_unchecked(Box::leak(new_node)) };
    let mut tail_prev = unsafe { self.tail.as_mut() }.prev.unwrap();

    unsafe {
      new.as_mut().prev = Some(tail_prev);
      tail_prev.as_mut().next = Some(new);
      new.as_mut().next = Some(self.tail);
      self.tail.as_mut().prev = Some(new);
    };

    self.len += 1;
  }
}

impl<T: Default> LinkedList<T> {
  pub fn pop_front(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }

    let mut front = unsafe { self.head.as_mut() }.next.unwrap();
    let mut new_front = unsafe { front.as_mut() }.next.unwrap();

    unsafe {
      self.head.as_mut().next = Some(new_front);
      new_front.as_mut().prev = Some(self.head);
    };

    self.len -= 1;

    unsafe { Box::from_raw(front.as_ptr()) }.value.into()
  }

  pub fn pop_back(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }

    let mut back = unsafe { self.tail.as_mut() }.prev.unwrap();
    let mut new_back = unsafe { back.as_mut() }.prev.unwrap();

    unsafe {
      self.tail.as_mut().prev = Some(new_back);
      new_back.as_mut().next = Some(self.tail);
    };

    self.len -= 1;

    unsafe { Box::from_raw(back.as_ptr()) }.value.into()
  }
}

impl<T: Default> LinkedList<T> {
  pub fn first(&self) -> Option<&T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let front = &self.head.as_ref().next.unwrap();
      Some(&front.as_ref().value)
    }
  }

  pub fn last(&self) -> Option<&T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let back = &self.tail.as_ref().prev.unwrap();
      Some(&back.as_ref().value)
    }
  }
}

impl<T: Default> LinkedList<T> {
  pub fn first_mut(&mut self) -> Option<&mut T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let front = &mut self.head.as_mut().next.unwrap();
      Some(&mut front.as_mut().value)
    }
  }

  pub fn last_mut(&mut self) -> Option<&mut T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      let back = &mut self.tail.as_mut().prev.unwrap();
      Some(&mut back.as_mut().value)
    }
  }
}

impl<T: Default> LinkedList<T> {
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

#[cfg(test)]
mod test_linkedlist {
  use super::*;

  #[test]
  fn add_elements() {
    let mut list = LinkedList::new();
    for i in [1, 2, 3] {
      list.push_front(i);
    }
    for i in [1, 2, 3] {
      list.push_back(i);
    }
    assert_eq!(list.len(), 6);
  }

  #[test]
  fn pop_elements() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    let mut collected = vec![];
    for _ in 0..list.len() {
      collected.push(list.pop_front().unwrap());
    }
    assert_eq!(collected, [3, 2, 1, 1, 2, 3]);
  }

  #[test]
  fn change_element() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.first_mut().unwrap() = 4;
    *list.last_mut().unwrap() = 4;
    let mut collected = vec![];
    for _ in 0..list.len() {
      collected.push(list.pop_front().unwrap());
    }
    assert_eq!(collected, [4, 2, 1, 1, 2, 4]);
  }

  #[test]
  fn iter() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.iter_mut().nth(1).unwrap() = 4;
    *list.iter_mut().nth(2).unwrap() = 5;
    let vec = list.iter().cloned().collect::<Vec<_>>();
    assert_eq!(vec, [3, 4, 5, 1, 2, 3]);
  }

  #[test]
  fn rev_iter() {
    let mut list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    *list.iter_mut().rev().nth(1).unwrap() = 4;
    *list.iter_mut().rev().nth(2).unwrap() = 5;
    let vec = list.iter().cloned().collect::<Vec<_>>();
    assert_eq!(vec, [3, 2, 1, 5, 4, 3]);
  }

  #[test]
  fn eq_and_clone() {
    let list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    let mut cloned = list.clone();
    assert_eq!(list, cloned);
    cloned.pop_front();
    assert_ne!(list, cloned);
  }

  #[test]
  fn format() {
    let list = LinkedList::from_iter([3, 2, 1, 1, 2, 3]);
    assert_eq!(format!("{}", list), "[3, 2, 1, 1, 2, 3]");
  }
}