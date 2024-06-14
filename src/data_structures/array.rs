pub fn run() {
  println!("Hello, world!");
  let mut arr = [1, 2, 3, 4, 5];

  for i in 0..arr.len() {
    println!("{}", arr[i]);
  }

  let mut arrVec = Vec::<i32>::new();
  arrVec.push(1);
  arrVec.push(2);
  arrVec.push(3);

  for element in arrVec.iter() {
    println!("{}", element);
  }
}