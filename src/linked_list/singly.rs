use crate::person::Person;

pub struct Node {
  pub value: Person,
  pub next: Option<Box<Node>>,
}

pub struct LinkedList {
  pub head: Option<Box<Node>>,
}

impl LinkedList {
  pub fn new() -> Self {
    LinkedList { head: None }
  }

  pub fn add(&mut self, value: Person) {
    let new_node = Box::new(Node { value, next: None });

    if self.head.is_none() {
      self.head = Some(new_node);
    } else {
      let mut current = self.head.as_mut().unwrap();
      while let Some(ref mut next_node) = current.next {
        current = next_node;
      }
      current.next = Some(new_node);
    }
  }
}
