use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() {
    let mut arguments = env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("the key is : {} and the value is {}", key, value);
    let mut database = Database::new().expect("Database new crashed ");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}
struct Database {
    map: HashMap<String, String>,
}
impl Database {
    fn new() -> Result<Database, io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        //read the kv.db file
        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let contents: String = fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }
        //parse string
        //populate map
        Ok(Database { map })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // creating a flush function
    fn flush(self) -> io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair);
        }
        fs::write("kv.db", contents)
    }
}
