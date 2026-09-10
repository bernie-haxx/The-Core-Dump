fn main() {
    let s1 = String::from("hello");

    let (s2, len) = len_calculate(s1);

    let boxer = Box::new(4);
    let uwu = 42;

    {
        let _t = (boxer, uwu);
    }

    let _x2 = uwu;

    println!("This string {s2} is of len:{len}");

    cache(&0, &mut 2);
}

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

fn len_calculate(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
