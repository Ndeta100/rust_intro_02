use std::env;
use std::fs;
fn main() {
    let mut arguments = env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("the key is : {} and the value is {}", key, value);

    // Writing to a file
    let contents = format!("{}\t{}\n", &key, &value);
    fs::write("kv.db", contents);
}
