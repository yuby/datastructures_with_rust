

#[derive(Debug, Clone)]
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
  fn add(&mut self, eq: &SparsePolynomial);
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

  fn add(&mut self, eq: &SparsePolynomial) {
    let mut resp_vec: Vec<SparsePolynomialTerm> = Vec::new();
    let mut max_index = 0;
    let mut is_run = true;
    let mut outer_index = 0;
    let mut inner_index = 0;

    while is_run {
      match (self.poly.get(outer_index), eq.poly.get(inner_index)) {
        (Some(outer), Some(inner)) => {
          if outer.exp > inner.exp {
            resp_vec.push(inner.clone());
            inner_index += 1;
          } else if outer.exp < inner.exp {
            resp_vec.push(outer.clone());
            outer_index += 1;
          } else {
            let mut new_term = outer.clone();
            new_term.coef = outer.coef + inner.coef;
            resp_vec.push(new_term);
            inner_index += 1;
            outer_index += 1;
          }
        },
        (Some(outer), None) => {
          resp_vec.push(outer.clone());
          outer_index += 1;
        },
        (None, Some(inner)) => {
          resp_vec.push(inner.clone());
          inner_index += 1;
        },
        (None, None) => is_run = false,
      }


      max_index += 1;
      if max_index > 100 {
        is_run = false;
        println!("BREAK");
      }
    }
    println!("loop count {:?}", max_index);
    self.poly = resp_vec;
  }
}

pub fn run() {
  let mut poly = SparsePolynomial::new();

  poly.add_term(SparsePolynomialTerm {
    coef: 2.0,
    exp: 5
  });
  poly.add_term(SparsePolynomialTerm {
    coef: 2.0,
    exp: 10
  });

  poly.print();

  let mut poly2 = SparsePolynomial::new();

  poly2.add_term(SparsePolynomialTerm {
    coef: 1.0,
    exp: 3
  });
  poly2.add_term(SparsePolynomialTerm {
    coef: 2.0,
    exp: 7
  });

  poly2.add_term(SparsePolynomialTerm {
    coef: 3.0,
    exp: 10
  });

  poly2.print();

  poly.add(&poly2);
  poly.print();
}