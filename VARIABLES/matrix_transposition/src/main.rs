/*
Instructions

    Define a struct named Matrix as a tuple of 2 tuples. The nested tuple will contain 2 i32s.

    Create a function named transpose that calculates the transposition of a 2x2 matrix.

pub fn transpose(m: Matrix) -> Matrix {
}

The transposition of a matrix, switches the columns to rows, and the rows to columns. For example:

( a b )   __ transposition __>   ( a c )
( c d )                          ( b d )

Matrix must implement Debug, PartialEq and Eq. You can use derive.

    Remember that you are defining a library, so any element that can be called from an external crate must be made public.

And its output:

$ cargo run
Original matrix Matrix((1, 3), (4, 5))
Transpose matrix Matrix((1, 4), (3, 5))
$

Notions:
    Defining a struct
    The Tuple Type
    Tuples
    Tuple Structs without Named Fields
    Adding Useful Functionality with Derived Traits
*/

use matrix_transposition::transpose;
use matrix_transposition::Matrix;

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}
