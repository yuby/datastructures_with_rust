fn fibonacci(n: i32) -> i32 {
  if n == 0 {
    return 0
  } else if n == 1{
    return 1
  }

  return fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibinacci_loop(n: i32) -> i32 {
  let mut sum = 0;
  let mut prev_n1 = 0;
  let mut prev_n2 = 0;

  for i in 0..(n + 1) {
    if i == 0 {
      prev_n1 = 0;
    } else if i == 1 {
      prev_n2 = 1;
    } else {
      sum = prev_n1 + prev_n2;
      prev_n1 = prev_n2;
      prev_n2 = sum;
    }
  }

  sum
}

pub fn run() {
  let res = fibonacci(10);

  println!("{}", res);

  let res2 = fibinacci_loop(10);

  println!("{}", res2);
}
