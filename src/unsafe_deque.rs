use std::ptr;

pub struct List<T> {
  head: Link<T>,
  tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
  prev: *mut Node<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List {
      head: None,
      tail: ptr::null_mut(),
    }
  }

  pub fn push_front(&mut self, elem: T) {
    let mut new_head = Box::new(Node {
      elem: elem,
      next: self.head.take(),
      prev: ptr::null_mut(),
    });

    let raw_head: *mut _ = &mut *new_head;

    match &mut new_head.next {
      None => {
        self.tail = raw_head;
      }

      Some(old_head) => {
        old_head.prev = raw_head;
      }
    }

    self.head = Some(new_head);
  }

  pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|mut old_head| {
      match old_head.next.take() {
        None => {
          self.tail = ptr::null_mut();
        }
        Some(mut new_head) => {
          new_head.prev = ptr::null_mut();
          self.head = Some(new_head);
        }
      }

      old_head.elem
    })
  }

  pub fn push_back(&mut self, elem: T) {
    let mut new_tail = Box::new(Node {
      elem: elem,
      next: None,
      prev: self.tail,
    });

    self.tail = &mut *new_tail;

    if new_tail.prev.is_null() {
      self.head = Some(new_tail);
    } else {
      unsafe {
        let prev: &mut Node<T> = &mut *new_tail.prev;
        prev.next = Some(new_tail);
      }
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    if self.tail.is_null() {
      return None;
    }

    #[allow(unused_assignments)]
    let mut pop_node: Option<Box<Node<T>>> = None;

    unsafe {
      if (*self.tail).prev.is_null() {
        pop_node = self.head.take();
        self.tail = ptr::null_mut();
      } else {
        let prev = &mut (*(*self.tail).prev);
        pop_node = prev.next.take();
        self.tail = &mut *prev;
      }
    }

    pop_node.map(|node| node.elem)
  }

  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
      &node.elem
    })
  }

  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
      &mut node.elem
    })
  }

  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter(&self) -> Iter<'_, T> {
    Iter { next: self.head.as_ref().map(|node| &**node) }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut { next: self.head.as_mut().map(|node| &mut **node) }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_node = self.head.take();
    while let Some(mut node) = cur_node {
      cur_node = node.next.take();
    }
  }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop_front()
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_ref().map(|next| &**next);
      &node.elem
    })
  }
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_mut().map(|node| &mut **node);
      &mut node.elem
    })
  }
}
  

#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics_front() {
    let mut list = List::new();

    assert!(list.pop_front().is_none());

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    list.push_front(4);
    list.push_front(5);

    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_front(), None);
  }

  #[test]
  fn basics_back() {
    let mut list = List::new();

    assert!(list.pop_back().is_none());

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));

    list.push_back(4);
    list.push_back(5);

    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), Some(4));

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_back(), None);
  }

  #[test]
  fn basics_queue_front() {
    let mut list = List::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), None);

    list.push_front(4);
    list.push_front(5);

    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), None);
  }

  #[test]
  fn basics_queue_back() {
    let mut list = List::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);

    list.push_back(4);
    list.push_back(5);

    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), None);
  }

  #[test]
  fn basics_queue_front_and_back() {
    let mut list = List::new();

    list.push_front(-2);

    assert_eq!(list.pop_back(), Some(-2));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);

    list.push_back(-1);

    assert_eq!(list.pop_front(), Some(-1));
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.pop_front(), None);

    list.push_back(1);
    list.push_front(2);
    list.push_back(3);
    list.push_front(4);

    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(1));

    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);

    list.push_back(5);
    list.push_front(6);
    list.push_back(7);
    list.push_front(8);

    assert_eq!(list.pop_back(), Some(7));
    assert_eq!(list.pop_front(), Some(8));
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_front(), Some(6));

    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
  }

  #[test]
  fn peek() {
    let mut list = List::new();

    assert!(list.peek().is_none());
    assert!(list.peek_mut().is_none());

    list.push_front(1);
    assert_eq!(list.peek(), Some(&1));
    assert_eq!(list.peek_mut(), Some(&mut 1));

    list.push_front(2);
    assert_eq!(list.peek(), Some(&2));
    assert_eq!(list.peek_mut(), Some(&mut 2));

    list.pop_front();
    assert_eq!(list.peek(), Some(&1));
    assert_eq!(list.peek_mut(), Some(&mut 1));

    list.pop_front();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();
    list.push_front(1); list.push_front(2); list.push_front(3);

    let mut iter = list.into_iter();

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push_front(1); list.push_front(2); list.push_front(3);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_mut() {
    let mut list = List::new();
    list.push_front(1); list.push_front(2); list.push_front(3);

    let mut iter = list.iter_mut();

    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), None);
  }
}
