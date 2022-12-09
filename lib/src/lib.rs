#[macro_export]
macro_rules! read_input_from_file {
    ($file:literal) => {{
        use std::io::Read;

        let path = format!("{}/{}", env!("CARGO_PKG_NAME"), $file);
        let mut input_file = std::fs::File::open(&path).expect("input file");
        let mut input = String::new();
        input_file.read_to_string(&mut input).expect("read input");
        input.trim_end().to_owned()
    }};
}

#[macro_export]
macro_rules! read_input {
    () => {
        lib::read_input_from_file!("input.txt");
    };
}

#[macro_export]
macro_rules! read_test_input {
    () => {
        lib::read_input_from_file!("test_input.txt");
    };
}
