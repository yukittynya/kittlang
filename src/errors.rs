pub fn error(line: i32, msg: String) {
    println!("[Line {}] {}", line, msg);
    panic!("Error");
}
