use rand::Rng;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<f64>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.gen()).collect())
            .collect();

        Matrix { rows, cols, data }
    }

    pub fn multiply(m1: &Matrix, m2: &Matrix) -> std::io::Result<Matrix> {
        if m1.cols != m2.rows {
            return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Number of columns of first matrix must be equal to number of rows of second matrix",
            ));
        }

        let mut result = Matrix::zeros(m1.rows, m2.cols);
        for i in 0..m1.rows {
            for j in 0..m2.cols {
                for k in 0..m1.cols {
                    result.data[i][j] += m1.data[i][k] * m2.data[k][j];
                }
            }
        }
        Ok(result)
    }

    pub fn add(m1: &Matrix, m2: &Matrix) -> std::io::Result<Matrix> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Number of rows and columns of both matrices must be equal",
            ));
        }

        let mut res = Matrix::zeros(m1.rows, m1.cols);
        for i in 0..m1.rows {
            for j in 0..m2.cols {
                res.data[i][j] = m1.data[i][j] + m2.data[i][j];
            }
        }

        Ok(res)
    }

    pub fn subtract(m1: &Matrix, m2: &Matrix) -> std::io::Result<Matrix> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Number of rows and columns of both matrices must be equal",
            ));
        }

        let mut res = Matrix::zeros(m1.rows, m1.cols);
        for i in 0..m1.rows {
            for j in 0..m2.cols {
                res.data[i][j] = m1.data[i][j] - m2.data[i][j];
            }
        }

        Ok(res)
    }

    pub fn dot_multiply(m1: &Matrix, m2: &Matrix) -> std::io::Result<Matrix> {
        if m1.rows != m2.rows || m1.cols != m2.cols {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Number of rows and columns of both matrices must be equal",
            ));
        }
        let mut res = Matrix::zeros(m1.rows, m1.cols);
        for i in 0..m1.rows {
            for j in 0..m2.cols {
                res.data[i][j] = m1.data[i][j] * m2.data[i][j];
            }
        }
        Ok(res)
    }

    pub fn map<F>(&mut self, func: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        Matrix::from(
            self.data
                .iter()
                .map(|row| row.iter().map(|col| func(col.to_owned())).collect())
                .collect(),
        )
    }

    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn multiply_test() {
        let m1 = Matrix {
            rows: 1,
            cols: 2,
            data: vec![vec![1.0, 2.0]],
        };

        let m2 = Matrix {
            rows: 2,
            cols: 3,
            data: vec![vec![2.0, 1.0, 5.0], vec![3.0, 4.0, 6.0]],
        };
        assert_eq!(
            Matrix::multiply(&m1, &m2).unwrap().data,
            vec![vec![8.0, 9.0, 17.0]]
        )
    }
}
