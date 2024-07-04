mod express;
use express::express_test;
fn main() {
    println!("Hello, world!");

    println!("rust learn");
    let y = express_test();
    println!("{}", y)
}