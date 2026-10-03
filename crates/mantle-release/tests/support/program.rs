fn main() {
    let name = std::env::current_exe().unwrap();
    println!("{} 0.1.4", name.file_name().unwrap().to_str().unwrap());
}
