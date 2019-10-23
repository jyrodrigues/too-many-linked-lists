use std::mem;

pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  pub fn push(&mut self, val: i32) {
    let new_node = Box::new(Node {
      elem: val,
      next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
  }
}

