fn main() {
    let str = String::from("hello");
    let len = calculate_length(&str);
    println!("{}", len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}