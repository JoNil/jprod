use std::{error::Error, fmt, fs, path::PathBuf, process::Command};

#[derive(Debug)]
enum PackerError {
    BuildFailed,
    CrinklerFailed,
}

impl Error for PackerError {
    fn description(&self) -> &str {
        match *self {
            PackerError::BuildFailed => "Build Failed",
            PackerError::CrinklerFailed => "Crinkler Failed",
        }
    }
}

impl fmt::Display for PackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn find_obj_files() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut result = Vec::new();

    if let Ok(rd) = fs::read_dir("../jprod/target/release/deps") {
        for entry in rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
        {
            let extension = entry.extension().unwrap().to_str().unwrap();

            if extension == "o" {
                result.push(entry.to_owned())
            }
        }
    }

    Ok(result)
}

fn run() -> Result<(), Box<dyn Error>> {
    let build_status = Command::new("cargo")
        .current_dir("../jprod/")
        .args(["build", "--release"])
        .status()?;

    if !build_status.success() {
        return Err(From::from(PackerError::BuildFailed));
    }

    let obj_files = find_obj_files()?;

    let crinkler_output = Command::new("../tools/crinkler.exe")
            .arg("/OUT:jprod.exe")
            .arg("/SUBSYSTEM:WINDOWS")
            .args(&obj_files)
            .arg("../lib/msvcrt-light-x64.lib")
            .arg("/LIBPATH:C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.32.31326/lib/x64")
            .arg("/LIBPATH:C:/Program Files (x86)/Windows Kits/10/Lib/10.0.19041.0/um/x64")
            .args(["kernel32.lib", "user32.lib", "opengl32.lib", "gdi32.lib"])
            .status()?;

    if !crinkler_output.success() {
        return Err(From::from(PackerError::CrinklerFailed));
    }

    Ok(())
}

pub fn main() {
    match run() {
        Ok(()) => println!("Success!"),
        Err(e) => println!("{}", e),
    }
}
