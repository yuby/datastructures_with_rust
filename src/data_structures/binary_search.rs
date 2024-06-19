fn binary_search(arr: &[i32], find_num: i32, start: usize, end: usize) -> Result<usize, i32> {
  if start > end {
    return Err(find_num);
  }

  // let middle_index = (((start + end) / 2) as f64).floor() as usize;
  let middle_index = (start + end) / 2;
  let try_find_num = arr[middle_index];

  if middle_index == start {
    return Err(find_num);
  }

  if try_find_num > find_num {
    return binary_search(&arr, find_num, start, middle_index);
  } else if try_find_num < find_num {
    return binary_search(&arr, find_num, middle_index, end);
  } else {
    return Ok(middle_index);
  }
}

pub fn run() {
  // let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let arr = [2, 4, 5, 5, 7, 8, 8, 10];

  match binary_search(&arr, 3, 0, arr.len()) {
    Ok(res) => {
      println!("found index: {}", res);
    },
    Err(res) => {
      println!("Fail to find num: {}", res);
    }
  }
}
