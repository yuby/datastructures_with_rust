fn recursion_sum(arr: &[i32], index: usize) -> i32 {
  if index > arr.len() - 1 {
    return 0
  }
  return arr[index] + recursion_sum(arr, index + 1)
}

pub fn run() {
  let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

  let resp = recursion_sum(&arr, 0);

  println!("SUM: {}", resp);
}
