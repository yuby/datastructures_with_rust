fn print_arr_to_str(arr: &[i32]) {
  let mut resp = String::from("");
  for index in 0..arr.len() {
    if arr[index] != 0 {
      let info = format!("{}{}", char::from((index + 97) as u8), arr[index]);

      resp += &info;
    }
  }

  println!("{}", resp);
}

pub fn run() {
  let mut sentence = String::from("ababcdfeeesdddaasx");
  let mut result: [i32; 26] = [0; 26];

  for bt in sentence.bytes() {
    let index =  (bt as usize) - 97;
    result[index] = result[index] + 1;
  }

  print_arr_to_str(&result);
}