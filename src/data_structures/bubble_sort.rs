use super::swap;

pub fn run() {
  let mut arr = [7, 4, 5, 1, 6, 9];

  for outer_index in 0..arr.len() {

    for inner_index in 0..(arr.len() - outer_index - 1) {
      let current_val = arr[inner_index];
      let next_val = arr[inner_index + 1];

      if (current_val > next_val) {
        swap::swap(&mut arr, inner_index, inner_index + 1);
        println!("> {:?}", arr);
      }
    }
    println!("==> {:?}", arr);
  }
}