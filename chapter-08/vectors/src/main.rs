#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

fn main() {
    let mut v = Vec::new();

    // updating
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    if let Some(third) = v.get(2) {
        println!("The third element is {}", third)
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100); // Good!

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // Can't mutate a vector with a existent immutable reference to it

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Works even with vectors accepting only values from the same type because for Rust, the type here is Vec<SpreadsheetCell>
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
