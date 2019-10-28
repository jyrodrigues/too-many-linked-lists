
pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let mut head = Box::new(Node {
      elem: elem,
      next: None,
    });

    self.head.take().map(|old_head| {
      head.next = Some(old_head);
    });

    self.head = Some(head);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|mut old_head| {
      self.head = old_head.next.take();

      old_head.elem
    })
  }
}


#[cfg(test)]
mod test {
  use super::List;

  #[test]
  fn basics() {
    let mut list = List::new();

    assert!(list.pop().is_none());

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
