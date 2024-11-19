#![allow(dead_code)]


//source https://github.com/mathletedev
//General Public License (GPL) wersja 3

//https://www.youtube.com/watch?v=FI-8L-hobDY
//38.18

use rand::thread_rng;
use rand::Rng;

use serde::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct Matrix{
    rows: usize,
    pub columns: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix{
    pub fn zeros(rows: usize, columns: usize) -> Matrix{
        Matrix{
            rows,
            columns,
            data: vec![vec![0.0; columns]; rows], //macierz zerowa
        } 
    }

    pub fn rand_matrix(rows: usize, columns: usize) -> Matrix{
        let mut rng = thread_rng();
        let mut result = Matrix::zeros(rows, columns);

        for i in 0..rows{
            for j in 0..columns{
                result.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        result
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
		Matrix {
			rows: data.len(),
			columns: data[0].len(),
			data,
		}
	}
    
    pub fn multiply(&mut self,other: &Matrix) -> Matrix{
        if self.columns != other.rows{
            panic!("This matrices cannot be multiplied - check their dimensions")
        }
        let mut result = Matrix::zeros(self.rows, other.columns);

        for i in 0..self.rows{
            for j in 0..other.columns{
                let mut sum = 0.0;
                for k in 0..self.columns{
                   sum+= self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    pub fn add(&mut self,other: &Matrix) -> Matrix{
        if self.rows != other.rows || self.columns != other.columns{
            panic!("This matrices cannot be added - check their dimensions")
        }
        let mut result = Matrix::zeros(self.rows, self.columns);

        for i in 0..self.rows{
            for j in 0..other.columns{
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    pub fn dot_multiply(&mut self,other: &Matrix) -> Matrix{
        if self.rows != other.rows || self.columns != other.columns{
            panic!("This matrices cannot be dot multiplied - check their dimensions")
        }
        let mut result = Matrix::zeros(self.rows, other.columns);

        for i in 0..self.rows{
            for j in 0..self.columns{
                result.data[i][j] = self.data[i][j]*other.data[i][j];
            }
        }
        result

    }

    pub fn substract(&mut self,other: &Matrix) -> Matrix{
        if self.rows != other.rows || self.columns != other.columns{
            panic!("This matrices cannot be substracted - check their dimensions")
        }
        let mut result = Matrix::zeros(self.rows, self.columns);

        for i in 0..self.rows{
            for j in 0..self.columns{
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result 
    }

    pub fn vec_to_matrix(data: Vec<Vec<f64>>) -> Matrix{
        Matrix{
            rows: data.len(),
            columns: data[0].len(),
            data,
           } 
    }

    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
    //iteracja po row, iterator po wartościach w wierszu, stosuje function dla każdego value,wartości->wektor ,wiersze->macierz
		Matrix::from(
			(self.data)
				.clone()
				.into_iter()
				.map(|row| row.into_iter().map(|value| function(value)).collect())
				.collect(),
		)
	}

    pub fn transpose(&mut self) -> Matrix{
        let mut result = Matrix::zeros(self.columns, self.rows);
        for i in 0..self.rows{
            for j in 0..self.columns{
                result.data[j][i] = self.data[i][j];
            }
        }
        result
        }
    }



