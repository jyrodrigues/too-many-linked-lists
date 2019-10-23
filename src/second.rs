pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
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

// https://rust-unofficial.github.io/too-many-lists/second-iter.html
// Alternatives:
//  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
//  pub fn iter(& self) -> Iter<'_, T> {
  pub fn iter(& self) -> Iter<T> {
    // TODO test on next line `&'b node` with `pub fn iter<'a, 'b>`
    // and then test with `&'a node`.
    Iter { next: self.head.as_ref().map(|node| &**node) }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut { next: self.head.as_mut().map(|node| &mut **node) }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();

    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

// This works but isn't ideal: at any point in time we could call list.next()
// after a list.push(). So basically it'd be a different kind of list.pop().
//impl<T> Iterator for List<T> {
//  type Item = T;
//  fn next(&mut self) -> Option<Self::Item> {
//    self.pop()
//  }
//}
//
// So here is a proper impl:

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      // Alternative method w/ turbofish operator: `::<>` helping the compiler
      // with a hint.
      //self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
      &node.elem
    })
  }
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
  fn basics() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

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

  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|elem| {
      *elem = 432348;
    });

    assert_eq!(list.peek(), Some(&432348));
    assert_eq!(list.pop(), Some(432348));
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_mut() {
    let mut list = List::new();

    list.push(1);
    list.push(2);
    list.push(3);

    {
      let mut iter = list.iter_mut();

      assert_eq!(iter.next(), Some(&mut 3));
      assert_eq!(iter.next(), Some(&mut 2));
      assert_eq!(iter.next(), Some(&mut 1));
    }

    {
      let mut iter = list.iter_mut();
      while let Some(n) = iter.next() {
        *n += 1;
      }
    }

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
  }
}

