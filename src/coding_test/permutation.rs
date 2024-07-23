use crate::data_structures::swap::swap;


fn permutation(arr: &mut [i32], offset: usize, fix_idx: usize) {
  swap(arr, fix_idx, offset);

  println!("{} ,{:?}", offset, arr);
  if offset < arr.len() - 1 {
    return permutation(arr, offset, fix_idx + 1);
  }

  return permutation(arr, offset + 1, offset + 1);
}

pub fn run() {
  let mut arr = [1,2,3];

  permutation(&mut arr, 0, 0);
}

// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
