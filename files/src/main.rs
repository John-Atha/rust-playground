use std::{ fs::File, io::{ self, ErrorKind, Read } };

fn main() {
    let filenames = vec!["data/hello.txt", "data/haha.txt"];
    for filename in filenames {
        let filename_string = String::from(filename);
        read_file(&filename_string);
        read_file_short(&filename_string).unwrap();
    }
}

fn read_file(filename: &String) {
    println!("Reading the file {filename}:");
    let file_result = File::open(filename.clone());
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("\tFile {filename} was not found.");
                    return;
                }
                _ => {
                    println!("\tGeneric rrror while reading the file {filename}: {error}");
                    return;
                }
            }
    };
    let mut contents = String::new();
    let bytes_len_result = file.read_to_string(&mut contents);
    let bytes_len = match bytes_len_result {
        Ok(bytes_len) => bytes_len,
        Err(error) => {
            println!("\tError while reading the file {filename}: {error}");
            0
        }
    };
    println!("\tThe contents of the file {filename} are the following {bytes_len} bytes:");
    println!("\t{contents}");
}

fn read_file_short(filename: &String) -> Result<(), io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    println!("\tThe contents of the file {filename} are the following bytes:");
    println!("\t{contents}");
    Ok(())
}