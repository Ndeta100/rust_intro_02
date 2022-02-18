use std::env;
fn main() {
    let mut arguments = env::args().skip(1);
    let key = &arguments.next();
}
