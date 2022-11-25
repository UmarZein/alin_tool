use std::ops::*;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub m: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Sized
        + Add<Output = T>
        + Neg<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Default
        + ::std::fmt::Display
        + ::std::fmt::Debug,
    u8: Into<T>,
{
    pub fn multiply_matrix(&self, other: Self) -> Result<Self, String> {
        if !(other.m.len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        };
        if !(other.m[0].len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        }
        if !(self.m.len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        };
        if !(self.m[0].len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        }
        if !(self.m[0].len() == other.m.len()) {
            return Err("matrix multiplication should be between LxM and MxN matrices".to_string());
        }
        // a b c * (x y)
        // d e f * (z j)
        //         (k l)
        let mut m: Vec<Vec<T>> = vec![];
        for i in 0..self.m.len() {
            let mut buffer: Vec<T> = vec![];
            for j in 0..self.m[0].len() {
                let mut inner_buffer: T = 0u8.into();
                for k in 0..other.m[0].len() {
                    inner_buffer = inner_buffer + self.m[i][j] * other.m[j][k];
                }
                buffer.push(inner_buffer);
            }
            m.push(buffer);
        }
        Ok(Matrix { m })
    }
    pub fn multiply_scalar(&self, scalar: T) -> Result<Self, String> {
        if !(self.m.len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        };
        if !(self.m[0].len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        }
        let h = self.m.len();
        let w = self.m[0].len();
        let mut result: Vec<Vec<T>> = vec![];
        for i in 0..h {
            let mut buffer: Vec<T> = vec![];
            for j in 0..w {
                buffer.push(scalar * self.m[i][j]);
            }
            result.push(buffer);
        }
        Ok(Matrix { m: result })
    }
    /// returns new matrix without row r and column c.
    /// if matrix is not NxN where N>1, return None
    pub fn without_rc(&self, r: usize, c: usize) -> Result<Self, String> {
        if !(self.m.len() > 1) {
            return Err("all matrix length musn't be empty".to_string());
        };
        if !(self.m[0].len() > 1) {
            return Err("all matrix length musn't be empty".to_string());
        }
        let h = self.m.len();
        let w = self.m[0].len();
        if !(r < h && c < w) {
            return Err("r must be lower than h, and c must be lower than w".to_string());
        }
        let mut result: Vec<Vec<T>> = vec![];
        for i in 0..h {
            let mut buffer: Vec<T> = vec![];
            for j in 0..w {
                if (i != r) && (j != c) {
                    buffer.push(self.m[i][j]);
                }
            }
            if i != r {
                result.push(buffer);
            }
        }
        Ok(Matrix { m: result })
    }
    /// returns determinant of NxN matrix.
    /// if matrix is not NxN where N>0, return
    /// None
    pub fn determinant(&self) -> Result<T, String> {
        if !(self.m.len() > 0) {
            return Err("all matrix length musn't be empty".to_string());
        };
        let h = self.m.len();
        let w = self.m[0].len();
        if !(h == w) {
            return Err("matrix size must be NxN".to_string());
        }
        if h == 1 {
            return Ok(self.m[0][0]);
        } else if h == 2 {
            let a = self.m[0][0];
            let b = self.m[0][1];
            let c = self.m[1][0];
            let d = self.m[1][1];
            // println!("\t[[{a}, {b}]");
            // println!("\t[{c}, {d}]]");
            // println!("\t{a}*{c}-{b}*{d} = {}",a*c-b*d);
            return Ok(a * d - b * c);
        }
        let mut total: T = 0u8.into();
        for i in 0..w {
            let mut mul = self.m[0][i];
            if i % 2 == 1 {
                mul = -mul
            }
            let mut new_matrix: Vec<Vec<T>> = vec![];
            for j in 1..h {
                let mut buffer: Vec<T> = vec![];
                for k in 0..w {
                    if k != i {
                        buffer.push(self.m[j][k]);
                    }
                }
                new_matrix.push(buffer);
            }
            // println!("new_matrix = {new_matrix:?}");
            let new_matrix_det = Matrix { m: new_matrix }.determinant()?;
            // println!("total += {mul}*{new_matrix_det}");
            total = total + mul * new_matrix_det;
        }
        Ok(total)
    }
    pub fn cofactor(&self) -> Result<Self, String> {
        let h = self.m.len();
        let w = self.m[0].len();
        let mut res: Vec<Vec<T>> = vec![];
        for i in 0..h {
            let mut buffer: Vec<T> = vec![];
            for j in 0..w {
                if ((i + j) % 2) == 0 {
                    buffer.push(self.without_rc(i, j)?.determinant()?);
                } else {
                    buffer.push(-self.without_rc(i, j)?.determinant()?);
                }
            }
            res.push(buffer);
        }
        Ok(Matrix { m: res })
    }
    pub fn transpose(&self) -> Self {
        let h = self.m.len();
        let w = self.m[0].len();
        let mut res: Vec<Vec<T>> = vec![];
        for i in 0..h {
            let mut buffer: Vec<T> = vec![];
            for j in 0..w {
                buffer.push(self.m[j][i]);
            }
            res.push(buffer);
        }
        Matrix { m: res }
    }
    pub fn adjugate(&self) -> Result<Self, String> {
        return Ok(self.cofactor()?.transpose());
    }
    pub fn inverse(&self) -> Result<Self, String> {
        let inverse_det: T = 1u8.into() / self.determinant()?;
        let adjugate = self.adjugate()?;
        adjugate.multiply_scalar(inverse_det)
    }
}

impl<T> ::std::fmt::Display for Matrix<T>
where
    T: ::std::fmt::Display + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = "[".to_string();
        for (i, l1) in self.m.iter().enumerate() {
            for (j, l2) in l1.iter().enumerate() {
                if j == 0 {
                    s.push_str("[");
                }
                s.push_str(l2.to_string().as_str());
                if (j + 1) < l1.len() {
                    s.push_str(", ")
                } else {
                    s.push_str("]");
                }
            }
            if (i + 1) < l1.len() {
                s.push_str(",\n");
            } else {
                s.push_str("]");
            }
        }
        f.write_str(s.as_str())
    }
}
