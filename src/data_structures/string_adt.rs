pub trait MyStringADT {
  fn new(s: &str) -> Self;
  fn print(&self);
  fn is_empty(&self) -> bool;
  fn get_length(&self) -> usize;
  fn find_str(&self, s: &str) -> Option<usize>;
}

#[derive(Debug)]
struct MyString {
  data: Vec<char>,
  size: usize,
}

impl MyStringADT for MyString {
  fn new (s: &str) -> MyString {
    MyString {
      data: s.chars().collect(),
      size: s.len(),
    }
  }

  fn get_length(&self) -> usize {
    self.size
  }

  fn is_empty(&self) -> bool {
    self.size == 0
  }

  fn find_str(&self, word: &str) -> Option<usize> {
    let text: String = self.data.iter().collect();

    text.find(word)
  }

  fn print(&self) {
    for c in &self.data {
      print!("{}", c);
    }
    println!();
  }
}

pub fn run() {
  let my_string = MyString::new("hell ol olll llao oololo olla o!");

  my_string.print();

  println!("is Empty ? {:?}", my_string.is_empty());

  println!("string length {:?}", my_string.get_length());

  if let Some(index) = my_string.find_str("ool") {
    println!("find index in string {:?}", index);
  }
  
}