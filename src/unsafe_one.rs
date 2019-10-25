pub struct List<'a, T> {
  head: Link<T>,
  tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<'a, T> List<'a, T> {
  pub fn new() -> Self {
    List { head: None, tail: None }
  }

  pub fn push(&'a mut self, elem: T) {
    let new_tail = Box::new(Node {
      elem: elem,
      next: None,
    });

    let new_tail = match self.tail.take() {
      Some(old_tail) => {
        old_tail.next = Some(new_tail);
        old_tail.next.as_mut().map(|node| &mut **node)
      }
      None => {
        self.head = Some(new_tail);
        self.head.as_mut().map(|node| &mut **node)
      }
    };

    self.tail = new_tail;
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|head| {
      let head = *head;
      self.head = head.next;

      if self.head.is_none() {
        self.tail.take();
      }

      head.elem

/* My trial at this stuff */
//      match old_head.next.take() {
//        None => {
//          self.tail.take();
//        }
//        new_head => {
//          self.head = new_head;
//        }
//      }
//
//      old_head.elem
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
