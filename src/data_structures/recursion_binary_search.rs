fn recursion_binary_search(arr: &[i32], start: usize, end: usize, find: i32) {
  let center_index = (start + end) / 2;

  if start == center_index || end == center_index {
    println!("No EXIST");
    return;
  }

  if arr[center_index] == find {
    println!("FOUND {}", center_index);
    return;
  } else if arr[center_index] > find {
    return recursion_binary_search(&arr, start, center_index, find);  
  } else if arr[center_index] < find {
    return recursion_binary_search(&arr, center_index, end, find);  
  }
}

pub fn run() {
  let arr = [2, 4, 5, 5, 7, 8, 8, 10];
  let len = arr.len();

  recursion_binary_search(&arr, 0, len - 1, 7);
}