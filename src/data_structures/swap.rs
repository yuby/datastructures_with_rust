pub fn swap(arr: &mut [i32], a: usize, b: usize) {
  let temp = arr[a];

  arr[a] = arr[b];
  arr[b] = temp;
}

pub fn run() {
  let mut arr = [1, 2, 3, 4, 5];
  swap(&mut arr, 1, 2);

  println!("{:?}", arr);
}