use crate::cut::Cut;

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
        return result;
    }

}


enum Regression {
    Global(PolyFunction2D),
    Split(Cut, PolyFunction2D, PolyFunction2D),
}

pub struct ChunkRegression {
    function: Regression,
    cost: f64,
}

impl ChunkRegression {
    pub fn new_global(degree: usize, coeffs: Vec<f64>, MSE: f64) -> Self {
        ChunkRegression { 
            function: Regression::Global(PolyFunction2D::from(degree, coeffs)), 
            cost: MSE + (2*degree) as f64 + 1.0
        }
    }
}