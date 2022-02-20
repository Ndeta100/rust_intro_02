use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() {
    let mut arguments = env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("the key is : {} and the value is {}", key, value);

    // Writing to a file
    let contents = format!("{}\t{}\n", &key, &value);
    fs::write("kv.db", contents).unwrap();
    let database = Database::new().expect("Database new crashed ");
}
struct Database {
    map: HashMap<String, String>,
}
impl Database {
    fn new() -> Result<Database, io::Error> {
        //read the kv.db file
        let contents = match fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Err(error);
            }
        };
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
        }
        //parse string
        //populate map
        Ok(Database {
            map: HashMap::new(),
        })
    }
}
