#[allow(unused_variables)]

fn main() {
    //// Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //// Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    //// Data types

    // INTEGER TYPES
    /* 8-bit	i8	    u8 */
    /* 16-bit	i16	    u16 */
    /* 32-bit	i32	    u32 */
    /* 64-bit	i64	    u64 */
    /* 128-bit	i128    u128 */
    /* arch	    isize	usize */
    let number: u32 = 255;
    println!("u32 value: {}", number);

    // Floating points
    let floatf64 = 2.0; // f64
    let floatf32: f32 = 3.0; // f32

    // Operations
    /* addition */
    let sum = 5 + 10;

    /* subtraction */
    let difference = 95.5 - 4.3;

    /* multiplication */
    let product = 4 * 30;

    /* division */
    let quotient = 56.7 / 32.2;

    /* remainder */
    let remainder = 43 % 5;

    //// Booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    //// Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //// Compound types
    /* Tuple */
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotation

    let (x, y, z) = tup; // destructuring

    println!("The value of y is: {}", y);

    let five_hundred = tup.0; // direct access
    println!("The value of five_hundred is: {}", five_hundred);

    /* Array */
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type and size annotation
    let a = [3; 5]; // [initial_value_of_each_element; size] outputs [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];
    //  let out_of_bounds = a[6]; // => RUST PANICKS ON OUT OF BOUNDS
}
