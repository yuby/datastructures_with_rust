use super::swap;

fn count(arr: &[i32], find_value: i32) -> i32 {
  let mut count_same_value = 0;

  for value in arr.iter() {
    if *value == find_value {
      count_same_value += 1;
    }
  }

  count_same_value
}

fn sorted_count(arr: &[i32], find_index: i32) -> i32 {
  if find_index == -1 {
    return -1
  }
  let mut count_same_value = 0;
  let target_value = arr[find_index as usize];

  for value in arr.iter() {
    if *value == target_value {
      count_same_value += 1;
    } else {
      if count_same_value > 0 {
        break;
      }
    }
  }

  count_same_value
}


fn pv_insertion_sort(arr: &mut [i32]) {
  for cnt in 1..arr.len() {
    let mut target_idx = cnt;
    for idx in (0..cnt).rev() {
      if arr[target_idx] < arr[idx] {
        swap::swap(arr, target_idx, idx);
        target_idx = idx;
      }
    }
  }
}

fn find_index_in_sorted_array(arr: &[i32], find_value: i32) -> i32 {
  let mut found_index = -1;

  for idx in 0..arr.len() {
    if arr[idx] == find_value {
      found_index = idx as i32;
      break;
    }
  }
  found_index
}

pub fn run() {
  let mut arr = [0, 1, 1, 3, 2, 5, 1, 2, 1, 1];

  println!("Count 3 : {}" , count(&arr, 3));
  println!("Count 1 : {}" , count(&arr, 1));

  let mut copied_arr = arr.clone();
  pv_insertion_sort(&mut copied_arr);

  println!("Count 3 : {}" , count(&copied_arr, 3));
  println!("Count 1 : {}" , count(&copied_arr, 1));


  println!("Count 3 : {}" ,
    sorted_count(
      &copied_arr,
      find_index_in_sorted_array(&copied_arr, 3),
    ),
  );
  println!("Count 1 : {}" , sorted_count(&copied_arr, 1));
}