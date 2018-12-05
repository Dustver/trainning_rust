use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Matrix (f64, f64, f64, f64);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", &self.0,&self.1,&self.2,&self.3)
    }
}

fn transpose(mtx: Matrix) -> Matrix {
    Matrix(mtx.0, mtx.2, mtx.1, mtx.3)
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}