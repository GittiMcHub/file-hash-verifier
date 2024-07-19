
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;

pub fn read_sum_file(path: PathBuf, filename: String) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read all lines into a vector
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    println!("Trying to find: {}", filename);

    match lines.len() {
        0 => return Err(Error::new(ErrorKind::Other, "File empty")), // Empty file
        1 => {
            let replaced_string = lines[0].clone().replace(&filename, "");
            let checksum = replaced_string.trim();
            Ok(checksum.to_string())
        }, // Single line
        _ => {
            println!("More than one row");
            // Find the line containing "ethxyz.iso"
            let target_line = lines.iter().find(|line| line.contains(&filename));
           

            match target_line {
                Some(line) => {
                    // assume its always "aefchecksumd0 filename.extension"
                    let replaced_string = line.clone().replace(&filename, "");
                    let checksum = replaced_string.trim();
                    Ok(checksum.to_string())
                },
                None => return Err(Error::new(ErrorKind::Other, "Filestring not found")), // No matching line found
            }
        }
    }
}

