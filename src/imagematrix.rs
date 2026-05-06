use nalgebra::{self as na, DMatrix, U2, DVector};
use std::vec::Vec;


pub fn create_matrix(width: u32, height: u32) -> DMatrix::<f64> {
    
    let mut v: Vec<f64> = Vec::with_capacity((height*width*3) as usize);
    for n in 0..(height*width){
        v.push(1.0);
        // println!("1");
    }
    for n in 0..(height*width){
        v.push((n/height) as f64);
        // println!("{0}", (n/height));
    }
    for n in 0..(height*width){
        v.push((n % width) as f64);
        // println!("{0}", (n % width));
    }
    //std::iter::&(0..height).collect();
    //let x: Vec<f64> = Vec::std::iter::&(0..height).iter().repeat(width).collect();
    //let y: Vec<f64> = Vec::std::iter::&(0..width).iter().collect();

    let a = DMatrix::<f64>::from_column_slice((width*height) as usize, 3 as usize, &v);
    // eprintln!("{}", a);
    a

}

// degree = 0 : corresponds to features of just adding a constant a
// degree = 1 : corresponds to features of bx + cy + a
// degree = 2 : corresponds to featueres of 
// need to use smthn to do this
pub fn create_matrix_n(width: u32, height: u32, n: u32) -> DMatrix::<f64> {
    
    let mut v: Vec<f64> = Vec::with_capacity((height*width*((2*n+1) as u32)) as usize);
    for j in 0..(height*width){
        v.push(1.0);
    }

    for deg in 1..=n {
        for term in 0..=deg {
            for g in 0..(height*width){
                v.push((((g/height) as f64).powi((deg-term) as i32) * ((g % width) as f64).powi(term as i32)) as f64);
                // println!("{0} ahhh", (((g/height).powi((deg-term) as i32) *(g % width)).powi(term as i32)) as f64);
            }
        }
    }

    // for k in 1..(n+1){
    //     for i in 0..(height*width){
    //         v.push(((i/height).pow(k as u32)) as f64);
    //         println!("{0}", ((i/height).pow(k as u32)));

    //     }
    //     if n > 1 {
    //     //for deg in 2..(n+1){
    //         for g in 0..(height*width){
    //             v.push((((g/height).pow(k-1) *(g % width)).pow(k)) as f64);
    //             println!("{0} ahhh", (((g/height).pow((k-1) as u32) *(g % width)).pow((k) as u32)));
    //         }
    //     //}
    //     }
    //     for p in 0..(height*width){
    //         v.push(((p % width).pow(k as u32)) as f64);
    //         println!("{0}", (p % width).pow(k as u32));
    //     }
        
    // }
    //std::iter::&(0..height).collect();
    //let x: Vec<f64> = Vec::std::iter::&(0..height).iter().repeat(width).collect();
    //let y: Vec<f64> = Vec::std::iter::&(0..width).iter().collect();

    let a = DMatrix::<f64>::from_column_slice((width*height) as usize, ((n + 1) * (n + 2) / 2) as usize, &v);
    // println!("{}", a);
    a

}


pub fn matrixOp(A: DMatrix<f64>) -> DMatrix<f64>{
    // (A^T A)^-1 A^T
    let temp = &A.clone();
    let temp1 = DMatrix::transpose(temp)* temp;
    temp1.try_inverse();
    let temp2 = temp * DMatrix::transpose(&A);
    println!("{}",temp2);
    temp2
}

pub fn matVecMult(A: DMatrix<f64>, V: DMatrix<f64>)-> DMatrix<f64>{
    let results = &A * &V;
    println!("{}", results);
    results

}

pub fn create_regression_matrix(X: &DMatrix<f64>) -> Option<DMatrix<f64>> {

    let Xt = X.transpose();
    let temp1 = &Xt * X;
    let inv = temp1.pseudo_inverse(0.00001).ok()?;
    let ret = inv * Xt;
    Some(ret)
}



pub struct MatPack {
    order_mats: Vec<DMatrix<f64>>,
    reg_mats: Vec<DMatrix<f64>>,
    pub max_deg: u32,
}

impl MatPack {
    pub fn new(size: usize, max_degree: u32) -> Self {
        let mut orders: Vec<DMatrix<f64>> = Vec::new();
        let mut regs: Vec<DMatrix<f64>> = Vec::new();
        for d in 0..=max_degree {
            let ord = create_matrix_n(size as u32, size as u32, d);
            if let Some(reg) = create_regression_matrix(&ord) {
                regs.push(reg);
            } else {
                panic!("Failed to create regression matrix");
            }
            orders.push(ord);
        }
        MatPack {
            order_mats: orders,
            reg_mats: regs,
            max_deg: max_degree,
        }
    }

    pub fn get(&self, deg: u32) -> (&DMatrix<f64>, &DMatrix<f64>) {
        if deg > self.max_deg {
            panic!("Requested degree out of bounds");
        }
        (&self.reg_mats[deg as usize], &self.order_mats[deg as usize])
    }
}