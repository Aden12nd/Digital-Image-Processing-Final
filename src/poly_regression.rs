


struct PolyRegression2D {
    degree: usize,
    coeffs: Vec<f64>
}

impl PolyRegression2D {
    pub fn from(degree: usize, coeffs: Vec<f64>) -> Self {
        // Assert that the number of coefficients is correct for the degree of the polynomial
        assert_eq!(coeffs.len(), (degree + 1) * (degree + 2) / 2);
        return PolyRegression2D { degree, coeffs };
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
        return result;
    }

}
