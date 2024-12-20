pub fn run() {
  let mut dynamic_2d_array: Vec<Vec<f64>> = vec![vec![0.0; 4]; 3];

  for i in 0..dynamic_2d_array.len() {
    for j in 0..dynamic_2d_array[i].len() {
      println!("{}", dynamic_2d_array[i][j]);
    }
  }
}