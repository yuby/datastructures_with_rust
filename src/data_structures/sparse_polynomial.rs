

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
  fn add_term(term: SparsePolynomialTerm);
}

impl SparsePolynomialHelper for SparsePolynomial {
  fn add_term(term: SparsePolynomialTerm) {}
}

pub fn run() {
  let poly = SparsePolynomial {
    poly: vec![]
  };

  poly.add_term(SparsePolynomialTerm {
    coef: 1.0,
    exp: 1
  });
  poly.add_term(SparsePolynomialTerm {
    coef: 2.0,
    exp: 2
  });
}