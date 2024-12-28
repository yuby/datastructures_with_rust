use std::fmt::Debug;

trait SparseMatrixHelpers<T> {
  fn new(rows: usize, cols: usize, capacity: usize) -> Self;
  fn set(&mut self, row: usize, col: usize, value: T);
  fn print_matrix(&self);
}

#[derive(Debug, Clone)]
struct SparseMatrixTerm<T> {
  row: usize,
  col: usize,
  value: T,
}
impl<T: Default> Default for SparseMatrixTerm<T> {
  fn default() -> Self {
      SparseMatrixTerm {
          row: 0,
          col: 0,
          value: T::default(),
      }
  }
}

#[derive(Debug)]
struct SparseMatrix<T> {
  num_rows: usize,
  num_cols: usize,
  capacity: usize,
  elements: Vec<SparseMatrixTerm<T>>,
  num_elements: usize,
}

impl<T: Debug + Default + Clone> SparseMatrixHelpers<T> for SparseMatrix<T> {
  fn new(rows: usize, cols: usize, capacity: usize) -> Self {
    // 1. with_capacity로 공간만 확보
    let mut elements = Vec::with_capacity(capacity);

    SparseMatrix {
      num_rows: rows,
      num_cols: cols,
      num_elements: 0,
      capacity,
      elements,
    }
  }
  fn set(&mut self, row: usize, col: usize, value: T) {
    let new_term  = SparseMatrixTerm {
      row,
      col,
      value
    };
    for val in self.elements.iter_mut() {
      if val.col == col && val.row == row {
        *val = new_term.clone();
        return;
      }
    }
    self.elements.push(new_term);
    self.num_elements += 1;

    assert!(self.num_elements == self.elements.len(), "check number of elements");
  }
  fn print_matrix(&self) {
    for row in 0..self.num_rows {
      for col in 0..self.num_cols {
        let val = self.elements.iter().find(|x| x.row == row && x.col == col);
        match val {
          Some(x) => print!("{:?} ", x.value),
          None => print!("0 "),
        }
      }
      println!("");
    }
  }
}

pub fn run() {
  let mut sparse_matrix = SparseMatrix::<i32>::new(3, 3, 3);

  sparse_matrix.set(0, 0, 1);
  sparse_matrix.set(1, 1, 1);
  sparse_matrix.set(2, 2, 1);
  sparse_matrix.set(2, 2, 3);

  sparse_matrix.print_matrix();
}
