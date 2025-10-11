use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::*;

// Only worrying about linux for now,
// but windows and mac could certainly be in
// the future
pub enum OS {
    Linux,
}

// shouldn't be public in the long term
pub struct ApplicationList {
    // likely will be more elaborate than just
    // this vec, so making a struct for future
    // proofing
    pub applications: Vec<String>,
}

// shouldn't be public in the long term
pub fn build_app_list() -> Result<ApplicationList, io::Error> {
    let search_path;
    match env::consts::OS {
        "linux" => {
            search_path = Path::new("/usr/bin/");
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Operating system either not supported by this program, or failed to be detected.",
            ));
        }
    }

    let mut app_vec: Vec<String> = Vec::new();

    for result in fs::read_dir(search_path)? {
        let result = String::from(result?.file_name().to_str().unwrap()); // if result exists unwrap
        app_vec.push(result);
    }

    Ok(ApplicationList {
        applications: app_vec,
    })
}
