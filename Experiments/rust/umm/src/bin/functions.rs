fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labelled_measurments(5, 'h');

    assignment_example();

    let x = five();

    println!("The function references to a scalar: {x}");
}

fn print_labelled_measurments(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("The number inserted is {x}");
}

fn assignment_example() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}


// functions with return values
fn five() -> i32 {
    5
}
