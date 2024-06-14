use super::swap;

pub fn run() {
  let mut arr = [7, 4, 5, 1, 6, 9];

  for index in 0..arr.len() {
    let mut min_idx = index;
    for next_idx in (index + 1)..arr.len() {
      if &arr[min_idx] > &arr[next_idx] {
        min_idx = next_idx;
      }
    }

    if min_idx != index {
      swap::swap(&mut arr, min_idx, index);
      println!("{:?}", arr);
    }
  }
}