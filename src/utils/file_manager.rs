use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};

pub fn open_or_create(path: &Path, default_value: String) -> String {
    if path.exists() {
        let mut contents = String::new();
        File::open(path)
            .and_then(|mut f| f.read_to_string(&mut contents))
            .expect("Failed to read config file");
        contents
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent dirs");
        }

        let mut file = File::create(path).expect("Failed to create config file");
        file.write_all(default_value.as_bytes())
            .expect("Failed to write default value");
        default_value
    }
}

/// # Description
/// Open a TOML config file, or create it with a default value.
/// - If the file exists and contains valid TOML → return it as `Value`.
/// - If the file exists but is invalid → overwrite with `default_value` and return it.
/// - If the file does not exist → create it with `default_value` and return it.
pub fn open_or_create_toml<'a, T>(path: &Path, default_value: T) -> T
where
    T: Serialize + DeserializeOwned,
{
    if path.exists() {
        let mut contents = String::new();
        match File::open(path).and_then(|mut f| f.read_to_string(&mut contents)) {
            Ok(_) => match toml::from_str::<T>(&contents) {
                Ok(val) => val,
                Err(e) => {
                    // invalid TOML → reset
                    println!("Overwriting invalid TOML: {:?}\n\n{}", e, contents);
                    write_default(path, &default_value);
                    default_value
                }
            },
            Err(_) => {
                // read error → reset
                println!("Overwriting bc failed to read");
                write_default(path, &default_value);
                default_value
            }
        }
    } else {
        // create parent dirs if needed
        if let Some(parent) = path.parent() {
            println!("Parent: {:?}", parent);
            fs::create_dir_all(parent).expect("Failed to create parent dirs");
        }
        write_default(path, &default_value);
        default_value
    }
}

fn write_default<T: Serialize>(path: &Path, value: &T) {
    let s = toml::to_string_pretty(value).expect("Failed to serialize default");
    let mut f = File::create(path).expect("Failed to create file");
    f.write_all(s.as_bytes()).expect("Failed to write file");
}
