fn fibonacci(n: i32) -> i32 {
  if n == 0 {
    return 0
  } else if n == 1{
    return 1
  }

  return fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn run() {
  let res = fibonacci(10);

  println!("{}", res);
}
