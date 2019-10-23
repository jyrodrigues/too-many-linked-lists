use std::rc::Rc;

pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
  elem: T,
  next: Link<T>,
}
