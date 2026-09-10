#[derive(Debug)]
#[allow(dead_code)]
enum SpreadsheetCell {
    Int(u64),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4];

    v.push(6);

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(does_not_exist) => println!("The number: {does_not_exist} does exist"),
        None => println!("The number does not exist"),
    }

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element"),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.3),
        SpreadsheetCell::Text(String::from("bule")),
    ];

    println!("{row:?}")
}
