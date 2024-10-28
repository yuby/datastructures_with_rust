use std::cmp::max;

trait PolynomialHelper {
    fn new(max: usize) -> Self;
    fn add_term(&mut self, coef: f32, exp: usize);
    fn print(&self);
    fn add(&mut self, poly: &Polynomial);
    fn multiply(&mut self, poly: &Polynomial);
    fn evaluation(&self, val: f32) -> f32;
}

#[derive(Debug)]
struct Polynomial {
    max_degree: usize,
    poly: Vec<f32>,
}

impl PolynomialHelper for Polynomial {
    fn new(max: usize) -> Polynomial {
        Polynomial {
            max_degree: max,
            poly: vec![0.0; max],
        }
    }
    fn add_term(&mut self, coef: f32, exp: usize) {
        self.poly[exp] = coef;
    }

    fn add(&mut self, poly2: &Polynomial) {
        let max_len = max(self.max_degree, poly2.max_degree);
        let mut new_poly = vec![0.0; max_len];

        for idx in 0..max_len {
            let coef1 = self.poly.get(idx).unwrap_or(&0.0);
            let coef2 = poly2.poly.get(idx).unwrap_or(&0.0);

            if *coef1 != 0.0 || *coef2 != 0.0 {
                new_poly[idx] = coef1 + coef2;
            }
        }

        self.poly = new_poly;
    }

    fn multiply(&mut self, poly2: &Polynomial) {
        let max_len = max(self.max_degree, poly2.max_degree);
        let mut new_poly = vec![0.0; max_len];

        for idx in 0..max_len {
            let coef1 = self.poly.get(idx).unwrap_or(&0.0);

            if *coef1 != 0.0 {
                for idx2 in 0..max_len {
                    let coef2 = poly2.poly.get(idx2).unwrap_or(&0.0);
                    if *coef2 != 0.0 {
                        new_poly[idx2 + idx] = (coef1 * coef2) + new_poly[idx2];
                    }
                }
            }
        }
        self.poly = new_poly;
    }

    fn evaluation(&self, value: f32) -> f32 {
        let mut sum = 0.0;
        for index in 0..self.max_degree {
            if self.poly[index] != 0.0 {
                sum = self.poly[index] * value.powi(index as i32) + sum;
            }
        }

        sum
    }

    fn print(&self) {
        let mut eq = String::from("");
        for index in 0..self.max_degree {
            if self.poly[index] != 0.0 {
                if index != 0 {
                    if !eq.is_empty() {
                        eq.push_str(" + ")
                    }
                }
                let mut str = String::new();
                if index == 0 {
                    str = format!("{}", self.poly[index]);
                } else {
                    str = format!("{} x^{}", self.poly[index], index);
                }

                eq.push_str(&str);
            }
        }
        println!("{}", eq);
    }
}

pub fn run() {
    let mut poly = Polynomial::new(100);
    let mut poly2 = Polynomial::new(100);

    poly.add_term(10.1, 1);
    poly.add_term(20.0, 0);
    poly.add_term(1.1, 4);

    poly2.add_term(10.1, 1);
    poly2.add_term(20.0, 0);
    poly2.add_term(1.1, 4);

    poly.add(&poly2);

    poly.print();

    println!("{:?}", poly.evaluation(1.0));

    let mut poly3 = Polynomial::new(10);
    let mut poly4 = Polynomial::new(10);
    poly3.add_term(1.0, 1);
    poly3.add_term(3.0, 4);

    poly4.add_term(1.0, 1);
    poly4.add_term(3.0, 3);

    poly3.multiply(&poly4);

    poly3.print();

    println!("{:?}", poly3.evaluation(3.0));
}
