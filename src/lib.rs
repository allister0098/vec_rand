#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use rand;
use rand::distributions::{Distribution, Standard};

pub struct RandVec {}

impl RandVec {
    pub fn generate<T>(len: usize) -> Vec<T> where Standard: Distribution<T> {
        let mut arr = Vec::new();
        for _ in 0..len {
            let v = rand::random::<T>();

            arr.push(v);
        }

        arr
    }

    pub fn generate_matrix<T>(rows: usize, columns: usize) -> Vec<Vec<T>>  where Standard: Distribution<T> {
        let mut matrix = Vec::new();
        for _ in 0..rows {
            matrix.push(RandVec::generate::<T>(columns));
        }

        matrix
    }
}
