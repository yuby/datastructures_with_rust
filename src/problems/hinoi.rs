fn hinoi(n: u32, from: &str, to: &str, aux: &str) {
  if n == 1 {
    println!("{} -> {}", from, to);
    return;
  }
  hinoi(n-1, from, aux, to);
  println!("{} -> {}", from, to);
  hinoi(n-1, aux, to, from);
}

pub fn run() {
  hinoi(5, "A", "C", "B");
}