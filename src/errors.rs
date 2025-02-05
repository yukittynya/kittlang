pub fn error(line: usize, msg: String) {
    println!("[Line {}] {}", line, msg);
    panic!("Error");
}
