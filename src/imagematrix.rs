use nalgebra::{self as na, DMatrix, U2, DVector};
use std::vec::Vec;

pub fn create_matrix(width: u32, height: u32) -> DMatrix::<f64> {
    
    let mut v: Vec<f64> = Vec::with_capacity((height*width*3) as usize);
    for n in 0..(height*width){
        v.push(1.0);
        println!("1");
    }
    for n in 0..(height*width){
        v.push((n/height) as f64);
        println!("{0}", (n/height));
    }
    for n in 0..(height*width){
        v.push((n % width) as f64);
        println!("{0}", (n % width));
    }
    //std::iter::&(0..height).collect();
    //let x: Vec<f64> = Vec::std::iter::&(0..height).iter().repeat(width).collect();
    //let y: Vec<f64> = Vec::std::iter::&(0..width).iter().collect();

    let a = DMatrix::<f64>::from_column_slice((width*height) as usize, 3 as usize, &v);
    a

}

// degree = 0 : corresponds to features of just adding a constant a
// degree = 1 : corresponds to features of bx + cy + a
// degree = 2 : corresponds to featueres of 
pub fn create_matrix_n(width: u32, height: u32) -> DMatrix::<f64> {
    
    let mut v: Vec<f64> = Vec::with_capacity((height*width*(2)) as usize);
    for n in 0..(height*width){
        v.push(1.0);
        println!("1");
    }
    for n in 0..(height*width){
        v.push((n/height) as f64);
        println!("{0}", (n/height));
    }
    for n in 0..(height*width){
        v.push((n % width) as f64);
        println!("{0}", (n % width));
    }
    //std::iter::&(0..height).collect();
    //let x: Vec<f64> = Vec::std::iter::&(0..height).iter().repeat(width).collect();
    //let y: Vec<f64> = Vec::std::iter::&(0..width).iter().collect();

    let a = DMatrix::<f64>::from_column_slice(3 as usize, (width*height) as usize, &v);
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

pub fn create_regression_matrix(X: DMatrix<f64>) -> Option<DMatrix<f64>> {

    let Xt = X.transpose();
    let temp1 = Xt.clone() * X;
    let inv = temp1.try_inverse()?;
    let ret = inv * Xt;
    Some(ret)
}