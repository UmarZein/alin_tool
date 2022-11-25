mod matrix;
use std::str::FromStr;

use matrix::*;

pub fn input<T>() -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
{
    let mut s: String = "".into();
    ::std::io::stdin().read_line(&mut s).unwrap();
    s.pop();
    s.parse::<T>()
}

pub fn prompted_input<T, S: ToString>(prompt: S) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr,
{
    let mut s: String = "".into();
    println!("{}", prompt.to_string());
    ::std::io::stdin().read_line(&mut s).unwrap();
    s.pop();
    s.parse::<T>()
}

pub fn matrix_input() -> Result<Matrix<f64>, <usize as FromStr>::Err> {
    let size: usize = until_ok(
        || prompted_input("size of matrix:"),
        "please enter a positive integer",
    );

    let mut m: Vec<Vec<f64>> = vec![];
    let mut retry = true;
    while retry {
        retry = false;
        m.clear();
        println!("enter whitespace-seperated matrix:");
        for _ in 0..size {
            let buffer: Vec<f64> = until_ok(
                || input::<String>(),
                "input a string which is a line of space-seperated numbers",
            )
            .split(" ")
            .filter_map(|x| x.parse::<f64>().ok())
            .collect();
            if buffer.len() != size {
                retry = true;
                println!("make sure matrix size is correct; non-numeric values are skipped");
                break;
            }
            m.push(buffer);
        }
    }
    return Ok(Matrix { m });
}

fn until_ok<O, E, F>(f: F, s: &str) -> O
where
    F: Fn() -> Result<O, E>,
{
    let mut res = f();
    loop {
        match res {
            Ok(x) => return x,
            _ => {
                println!("{}", s);
                res = f()
            }
        }
    }
}

fn main() {
    println!("ALIN tool:");
    loop {
        println!(
"0. exit
1. determinant 
2. cofactor
3. adjugate/adjoint
4. inverse
5. matrix multiplication"
        );
        match until_ok(input::<String>, "make sure you input a positive integer").as_str() {
            "0" => {break}
            "1" => {
                let m = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                match m.determinant() {
                    Ok(res) => println!("determinant:\n{res}"),
                    Err(s) => println!("error: {s}"),
                }
            }
            "2" => {
                let m = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                match m.cofactor() {
                    Ok(res) => println!("cofactor:\n{res}"),
                    Err(s) => println!("error: {s}"),
                }
            }
            "3" => {
                let m = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                match m.adjugate() {
                    Ok(res) => println!("adjugate/adjoint:\n{res}"),
                    Err(s) => println!("error: {s}"),
                }
            }
            "4" => {
                let m = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                match m.inverse() {
                    Ok(res) => println!("inverse:\n{res}"),
                    Err(s) => println!("error: {s}"),
                }
            }
            "5" => {
                let m1 = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                let m2 = until_ok(matrix_input, "invalid matrix; make sure matrix size is NxN");
                match m1.multiply_matrix(m2) {
                    Ok(res) => println!("A * B:\n{res}"),
                    Err(s) => println!("error: {s}"),
                }
            }
            _ => (),
        }
    }
}
