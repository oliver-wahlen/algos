use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let file_select =  File::open("n.txt");
    let _ = match file_select {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("n.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error {e:?} creating file"),
            }, 
            _ => panic!("Problem open file: {error:?}"),
        },
    };
}

fn read_uname_file() -> Result<String, io::Error> {
    let uname_file_result = File::open("n.txt");

    let mut uname_file = match uname_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut uname = String::new();

    match uname_file.read_to_string(&mut uname) {
        Ok(_) => Ok(uname),
        Err(e) => Err(e),
    }
}

fn read_uname_file_shorter() -> Result<String, io::Error> {
    let mut uname_file = File::open("n.txt")?;
    let mut uname = String::new();
    uname_file.read_to_string(&mut uname)?;
    Ok(uname)
}
