pub mod direction;
pub mod pos;
pub mod pos3d;
pub mod vector;

#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::Read;

        let inpt_path = if std::env::args().find(|a| a == "--test").is_some() {
            "test_input.txt"
        } else {
            "input.txt"
        };

        let path = format!("{}/{}", env!("CARGO_PKG_NAME"), inpt_path);
        let mut input_file = std::fs::File::open(&path).expect("input file");
        let mut input = String::new();
        input_file.read_to_string(&mut input).expect("read input");
        input.trim_end().to_owned()
    }};
}

#[macro_export]
macro_rules! read_test_input {
    () => {{
        use std::io::Read;

        let inpt_path = "test_input.txt";

        let path = format!("{}/{}", env!("CARGO_PKG_NAME"), inpt_path);
        let mut input_file = std::fs::File::open(&path).expect("input file");
        let mut input = String::new();
        input_file.read_to_string(&mut input).expect("read input");
        input.trim_end().to_owned()
    }};
}
