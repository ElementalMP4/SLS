use std::{
    env,
    ffi::OsString,
    io::ErrorKind,
    path::{Path, PathBuf},
};

const RED: &str = "\x1b[31m"; //Red foreground
const YELLOW: &str = "\x1b[33m"; //Yellow foreground
const RESET: &str = "\x1b[0m"; //Set back to default

const BYTE: u64 = 1_000;
const KILOBYTE: u64 = 1_000_0000;
const MEGABYTE: u64 = 1_000_0000_0000;
const GIGABYTE: u64 = 1_000_0000_0000_0000;
const TERABYTE: u64 = 1_000_0000_0000_0000_0000;

fn get_path_name(path: &PathBuf) -> OsString {
    match path.file_name() {
        Some(name) => name.to_os_string(),
        None => OsString::from(".."),
    }
}

fn print_dir_name(path: &PathBuf) {
    println!(
        "{}{}{}",
        YELLOW,
        get_path_name(path).to_string_lossy(),
        RESET
    );
}

fn get_file_size(path: &PathBuf) -> Result<String, std::io::Error> {
    let metadata = path.metadata()?;
    match metadata.len() {
        len if len < BYTE => Ok(format!("{}B", len)),
        len if len < KILOBYTE => Ok(format!("{:.2}KB", len as f32 / BYTE as f32)),
        len if len < MEGABYTE => Ok(format!("{:.2}MB", len as f32 / KILOBYTE as f32)),
        len if len < GIGABYTE => Ok(format!("{:.2}GB", len as f32 / MEGABYTE as f32)),
        // added terabyte support
        len if len < TERABYTE => Ok(format!("{:.2}TB", len as f32 / GIGABYTE as f32)),
        _ => Ok(String::from("Thats a really big file 😳")),
    }
}

fn print_file_name(path: &PathBuf) -> Result<(), std::io::Error> {
    let file_size = get_file_size(path)?;
    println!(
        "{}{} ({}){}",
        RED,
        get_path_name(path).to_string_lossy(),
        file_size,
        RESET
    );

    Ok(())
}

fn list_dir(path: &Path) -> Result<(), std::io::Error> {
    for entry in path.read_dir()? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            print_dir_name(&entry.path());
        } else {
            print_file_name(&entry.path())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let curr = env::current_dir().expect("Error fetching current directory");
    let mut path = curr.as_path();

    if args.len() == 2 {
        path = Path::new(&args[1]);
    }

    if !path.exists() {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            format!("Path `{}` does not exist", path.display()),
        ));
    }

    list_dir(path)?;

    Ok(())
}
