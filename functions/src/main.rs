fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // expressions
    let y = {
        let x = 3;
        // notice the lack of a semicolon
        // this is an expression, not a statement
        // expressions return values
        // statements do not
        x + 1
    };

    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");

    let w = plus_one(5);
    println!("The value of w is: {w}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // notice the lack of a semicolon
    5
}

fn plus_one(x: i32) -> i32 {
    // notice the lack of a semicolon
    x + 1
}
