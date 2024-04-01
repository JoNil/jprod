use std::{error::Error, fmt, process::Command};

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

fn run() -> Result<(), Box<dyn Error>> {
    let build_status = Command::new("cargo")
        .current_dir("../jprod/")
        .args(["build", "--release"])
        .status()?;

    if !build_status.success() {
        return Err(From::from(PackerError::BuildFailed));
    }

    let output = Command::new("tools/squishy-x64.exe")
        .args(["-o", "jprod.exe"])
        .args(["-i", "target/release/jprod.exe"])
        .status()?;

    if !output.success() {
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
