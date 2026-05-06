use crate::cut::Cut;

#[derive(Clone)]
pub struct PolyFunction2D {
    degree: usize,
    coeffs: Vec<f64>
}


impl PolyFunction2D {
    pub fn from(degree: usize, coeffs: Vec<f64>) -> Self {
        // Assert that the number of coefficients is correct for the degree of the polynomial
        assert_eq!(coeffs.len(), (degree + 1) * (degree + 2) / 2);
        return PolyFunction2D { degree, coeffs };
    }

    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let mut result = self.coeffs[0]; // Start with the constant term
        let mut coeff_idx = 1;
        if self.degree > 0 {
            result += self.coeffs[1] * x; // x term
            result += self.coeffs[2] * y; // y term
            coeff_idx = 3;
        }
        for deg in 2..=self.degree {
            for term in 0..=deg {
                result += self.coeffs[coeff_idx] * x.powi((deg-term) as i32) * y.powi(term as i32);
                coeff_idx += 1;
            }
        }
        // c
        // x y
        // x2 + y2 + xy
        // x3 + x2y + xy2 + y3
        // x4 + x3y + x2y2 +xy3 + y4
        // println!("Result: {}", result);
        return result;
    }

}


#[derive(Clone)]
enum Regression {
    Global(PolyFunction2D),
    Split(Cut, PolyFunction2D, PolyFunction2D),
}

#[derive(Clone)]
pub struct ChunkRegression {
    function: Regression,
    pub(crate) cost: f64,
}

impl ChunkRegression {

    pub fn new_empty() -> Self {
        ChunkRegression {
            function: Regression::Global(PolyFunction2D::from(0, vec![0.0])),
            cost: f64::INFINITY,
        }
    }

    pub fn new_global(degree: usize, coeffs: Vec<f64>, MSE: f64) -> Self {
        ChunkRegression { 
            function: Regression::Global(PolyFunction2D::from(degree, coeffs)), 
            cost: MSE + (degree) as f64 + 1.0
        }
    }

    pub fn new_split(degree: usize, coeffs_a: Vec<f64>, coeffs_b: Vec<f64>, size: usize, cut: Cut, MSE: f64) -> Self {
        ChunkRegression { 
            function: Regression::Split(
                cut,
                PolyFunction2D::from(degree, coeffs_a), 
                PolyFunction2D::from(degree, coeffs_b) 
            ), 
            // cost = MSE + 4*degree + ln(size^2) + 2
            cost: MSE + (4*degree) as f64 + 2.0*(size as f64).ln() + 2.0 
        }
    }

    pub fn predict(&self, x: f64, y: f64) -> f64 {
        match &self.function {
            Regression::Global(fun) => fun.eval(x, y),
            Regression::Split(_, _, _) => panic!("Does not handle splits"),
        }
    } 
}