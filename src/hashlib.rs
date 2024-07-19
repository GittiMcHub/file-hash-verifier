use sha2::{Digest, Sha256, Sha512};
use md5;
use strum::EnumString;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;


#[derive(EnumString)]
pub enum HashAlgorithm {
    MD5,
    SHA256,
    SHA512,
}

pub fn hash(path: &PathBuf, algorithm: HashAlgorithm) -> Result<String, std::io::Error> {
    match algorithm {
        HashAlgorithm::MD5 => {
            let digest = md5::compute(std::fs::read(path)?);
            Ok(format!("{:x}", digest))
        }
        HashAlgorithm::SHA256 => {
            let input = File::open(path)?;
            let mut reader = BufReader::new(input);
            let mut hasher = Sha256::new();
            let mut buffer = [0; 1024];

            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }

            let digest = hasher.finalize();
            Ok(format!("{:x}", digest))
        }
        HashAlgorithm::SHA512 => {
            let input = File::open(path)?;
            let mut reader = BufReader::new(input);
            let mut hasher = Sha512::new();
            let mut buffer = [0; 1024];

            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }

            let digest = hasher.finalize();
            Ok(format!("{:x}", digest))
        }
    }
}