use super::swap;

pub fn run() {
  let mut arr = [7, 4, 1, 5, 6, 9];

  for cnt in 1..arr.len() {
    let mut target_idx = cnt;
    for idx in (0..cnt).rev() {
      if arr[target_idx] < arr[idx] {
        swap::swap(&mut arr, target_idx, idx);
        target_idx = idx;
        println!("inside {:?}", arr);
      }
    }
    println!("{:?}", arr);
  }
}