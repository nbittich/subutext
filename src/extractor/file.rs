use std::{
    fs,
    io::{Error, ErrorKind},
};

pub fn read_file<T>(args: T) -> Result<String, std::io::Error>
where
    T: Iterator<Item = String>,
{
    match args.skip(1).next() {
        Some(file_path) => Ok(fs::read_to_string(file_path)?),
        None => Err(Error::new(ErrorKind::NotFound, "File not found!")),
    }
}

#[cfg(test)]
mod test {
    use super::read_file;

    #[test]
    fn read_file_not_found() {
        let result = read_file([""].iter().map(|s| s.to_string()));
        result.expect_err("File not found!");
    }
    #[test]
    fn read_file_test() {
        const FILE_PATH: &str = "/tmp/hello_world.txt";
        let args = ["path_to_app", FILE_PATH];

        println!(
            "File doesn't exist: {}",
            std::fs::remove_file(FILE_PATH).is_err()
        );
        std::fs::write(FILE_PATH, "Hello world!").unwrap();
        let result = super::read_file(args.iter().map(|s| s.to_string()));
        match result {
            Ok(the_result) => assert_eq!("Hello world!", the_result),
            Err(e) => panic!(e),
        }
    }
}
