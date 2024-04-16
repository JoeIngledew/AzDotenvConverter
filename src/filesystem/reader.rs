use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

// Currently unused
// pub fn get_file_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

pub fn get_file_text(input: &str) -> Result<String, io::Error> {
    let path = Path::new(input);
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
