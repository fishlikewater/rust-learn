fn main() {
    let str = String::from("hello");
    let len = calculate_length(&str);
    println!("{}, {}", str, len);

    let mut str1 = String::from("hello");
    push_str_test(&mut str1);
    print!("{}", str1);
    no_use();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn push_str_test(s: &mut String) {
    s.push_str(", world")
}

fn no_use() {
    let mut s = String::from("hello");

    let _r1 = &s;
    let _r2 = &s;
    let _r3 = &mut s;
}