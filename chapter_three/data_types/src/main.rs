fn main() {
    ///
    /// FLOATING POINT NUMBERS
    ///
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    ///
    /// NUMERIC OPERATIONS
    ///
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    ///
    /// BOOL
    ///
    let t = true;
    let f: bool = false; // with explicit type annotation

    ///
    /// CHAR
    ///
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    ///
    /// TUPLE
    ///
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
