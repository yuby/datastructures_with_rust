#[derive(Debug)]
struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
  fn new(data: T) -> Self {
    Node {
      data,
      next: None
    }
  }
}

#[derive(Debug)]
pub struct LinkedList<T> {
  head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList {
      head: None
    }
  }
  pub fn prepend(&mut self, data: T) {
    let mut new_node = Box::new(Node::new(data));

    match self.head.take() {
      Some(old_head) => {
        new_node.next = Some(old_head);
        self.head = Some(new_node);
      },
      None => {
        self.head = Some(new_node);
      }
    }
  }
}

pub fn run() {
  let mut linked_list = LinkedList::<i32>::new();

  linked_list.prepend(1);
  linked_list.prepend(2);
  linked_list.prepend(3);

  println!("{:?}", linked_list);
}