

#[derive(Debug)]
struct SparsePolynomialTerm {
  coef: f32,
  exp: i32,
}

#[derive(Debug)]
struct SparsePolynomial {
  poly: Vec<SparsePolynomialTerm>,
}

trait SparsePolynomialHelper {
  fn new() -> Self;
  fn add_term(&mut self, term: SparsePolynomialTerm);
  fn print(&self);
}

impl SparsePolynomialHelper for SparsePolynomial {
  fn new() -> SparsePolynomial {
    SparsePolynomial {
      poly: Vec::new()
    }
  }
  fn add_term(&mut self, term: SparsePolynomialTerm) {
    self.poly.push(term);
  }
  fn print(&self) {
    let mut eq = String::from("");

    for (index, term) in self.poly.iter().enumerate() {
      if index != 0 {
        if !eq.is_empty() {
            eq.push_str(" + ")
        }
      }
      eq.push_str(&format!("{} x^{}", term.coef, term.exp));
    }

    println!("{:?}", eq);
  }
}

pub fn run() {
  let mut poly = SparsePolynomial::new();

  poly.add_term(SparsePolynomialTerm {
    coef: 1.0,
    exp: 1
  });
  poly.add_term(SparsePolynomialTerm {
    coef: 2.0,
    exp: 10
  });

  poly.print();
}