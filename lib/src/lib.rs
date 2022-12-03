#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::Read;

        let path = format!("{}/input.txt", env!("CARGO_PKG_NAME"));
        let mut input_file = std::fs::File::open(&path).expect("input file");
        let mut input = String::new();
        input_file.read_to_string(&mut input).expect("read input");
        input
    }};
}
