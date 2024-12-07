#[derive(Debug, Clone)]
struct MatrixTerm {}

#[derive(Debug)]
struct Matrix {
  num_rows: i32,
  num_cols: i32,
  values: Vec<f64>,
}

trait MatrixHelper {
  fn new(num_rows: i32, num_cols: i32) -> Self;
  fn set_value(&mut self, num_rows: i32, num_cols: i32, value: f64);
  fn get_value(&self, num_rows: i32, num_cols: i32) -> f64;
  fn print(&self);
  fn add(&mut self, matrix: &Matrix);
  fn transpose(&self);
}

impl MatrixHelper for Matrix {
  fn new(num_rows: i32, num_cols: i32) -> Matrix {
    let values = vec![0.0; (num_rows * num_cols) as usize];
    Matrix {
      num_rows,
      num_cols,
      values,
    }
  }

  fn set_value(&mut self, row: i32, col: i32, value: f64) {
    let index = (col + self.num_cols * row) as usize;
    self.values[index] = value;
  }

  fn get_value(&self, row: i32, col: i32) -> f64{
    let index = (col + self.num_cols * row) as usize;
    self.values[index]
  }

  fn add(&mut self, matrix: &Matrix) {
    for idx in 0..(self.num_cols * self.num_rows) {
      self.values[idx as usize] += matrix.values[idx as usize];
    }
  }

  fn transpose(&self) {
    for col in 0..self.num_cols {
      for row in 0..self.num_rows {
        let index = col + self.num_cols * row;
        print!("{:?} ", self.values[index as usize]);
      }
      println!("");
    }
  }

  fn print(&self) {
    for row in 0..self.num_rows {
      for col in 0..self.num_cols {
        let index = col + self.num_cols * row;
        print!("{:?} ", self.values[index as usize]);
      }
      println!("");
    }
  }
}


pub fn run() {
  let mut mat = Matrix::new(3, 4);

  mat.set_value(0, 0, 1.0);
  mat.set_value(0, 1, 2.0);
  mat.set_value(0, 2, 3.0);
  mat.set_value(0, 3, 4.0);

  mat.set_value(1, 0, 5.0);
  mat.set_value(1, 1, 6.0);
  mat.set_value(1, 2, 7.0);
  mat.set_value(1, 3, 8.0);

  mat.set_value(2, 0, 9.0);
  mat.set_value(2, 1, 10.0);
  mat.set_value(2, 2, 11.0);
  mat.set_value(2, 3, 12.0);

  let mut mat2 = Matrix::new(3, 4);

  mat2.set_value(0, 0, 1.0);
  mat2.set_value(0, 1, 2.0);
  mat2.set_value(0, 2, 3.0);
  mat2.set_value(0, 3, 4.0);

  mat2.set_value(1, 0, 5.0);
  mat2.set_value(1, 1, 6.0);
  mat2.set_value(1, 2, 7.0);
  mat2.set_value(1, 3, 8.0);

  mat2.set_value(2, 0, 9.0);
  mat2.set_value(2, 1, 10.0);
  mat2.set_value(2, 2, 11.0);
  mat2.set_value(2, 3, 12.0);

  mat.add(&mat2);
  mat.print();
  mat.transpose();
}