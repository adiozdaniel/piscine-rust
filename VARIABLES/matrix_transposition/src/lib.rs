
// #debug
#[derive(Debug, PartialEq, Eq)] // needed for assert_eq! to work on Matrix instances

// struct Matrix as a tuple of 2 tuples
pub struct Matrix(pub(i32, i32), pub(i32, i32));

// function transpose that calculates the transposition of a 2x2 matrix
pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

// tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix = Matrix((1, 2), (3, 4));
        let transposed = Matrix((1, 3), (2, 4));
        assert_eq!(transpose(matrix), transposed);

        let matrix = Matrix((0, -1), (2, 3));
        let transposed = Matrix((0, 2), (-1, 3));
        assert_eq!(transpose(matrix), transposed);

        let matrix = Matrix((5, 6), (7, 8));
        let transposed = Matrix((5, 7), (6, 8));
        assert_eq!(transpose(matrix), transposed);
    }
}
