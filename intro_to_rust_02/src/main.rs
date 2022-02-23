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
    match database.flush() {
        Ok(()) => print!("yay"),
        Err(err) => print!("Oh no  Error {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
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
        Ok(Database { map, flush: false })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    // creating a flush function
    fn flush(&mut self) -> io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    fs::write("kv.db", contents)
}
