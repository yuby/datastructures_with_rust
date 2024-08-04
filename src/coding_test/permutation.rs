use crate::data_structures::swap::swap;

fn permutation(arr: &mut [i32], offset: usize) {
  if offset == arr.len() {
    println!("{:?}", arr);
  } else {
    for index in offset..arr.len() {
      swap(arr, offset, index);
      permutation(arr, offset + 1);
      swap(arr, offset, index);
    }
  }
}

pub fn run() {
  let mut arr = [1,2,3,4];

  permutation(&mut arr, 0);
}
