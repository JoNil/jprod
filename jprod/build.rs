use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Error, Read, Write},
    path::Path,
    process::Command,
    sync::Mutex,
};

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn write_file<P: AsRef<Path>>(path: P, data: &str) -> Result<(), Error> {
    let mut f = File::create(path)?;
    f.write_all(data.as_bytes())?;
    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct ShaderData {
    vertex_source: Option<String>,
    fragment_source: Option<String>,
}

impl ShaderData {
    fn new() -> ShaderData {
        ShaderData {
            vertex_source: None,
            fragment_source: None,
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=src/shaders");

    let shaders = Mutex::new(HashMap::new());

    let shaders = &shaders;
    let out_dir = &out_dir;

    if let Ok(rd) = fs::read_dir("src/shaders") {
        for entry in rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
        {
            let name = entry.file_stem().unwrap().to_str().unwrap().to_uppercase();
            let extension = entry.extension().unwrap().to_str().unwrap();

            if extension == "vert" || extension == "frag" {
                let minified_name = format!(
                    "{}/{}_min.{}",
                    out_dir,
                    entry.file_stem().unwrap().to_str().unwrap(),
                    extension
                );

                let minifier_output = Command::new("../tools/shader_minifier.exe")
                    .arg(entry.to_str().unwrap())
                    .arg("--format")
                    .arg("none")
                    .arg("--preserve-externals")
                    .arg("-o")
                    .arg(&minified_name)
                    .status()
                    .unwrap();

                if !minifier_output.success() {
                    panic!();
                }

                {
                    let mut shaders = shaders.lock().unwrap();

                    let shader = shaders.entry(name).or_insert(ShaderData::new());

                    if extension == "vert" {
                        shader.vertex_source = Some(read_file(&minified_name).unwrap());
                    } else if extension == "frag" {
                        shader.fragment_source = Some(read_file(&minified_name).unwrap());
                    }
                }
            }
        }
    }

    let shaders = &*shaders.lock().unwrap();

    {
        let mut shader_source = String::new();

        for (name, shader) in shaders {
            if let (Some(vertex), Some(fragment)) = (&shader.vertex_source, &shader.fragment_source)
            {
                #[allow(clippy::format_in_format_args)]
                shader_source.push_str(
                        &format!("pub static {}_VERT: &'static str = {};\n\npub static {}_FRAG: &'static str = {};\n\n",
                                name,
                                format!(r###"r##"{}"##"###, vertex),
                                name,
                                format!(r###"r##"{}"##"###, fragment)));
            }
        }

        write_file(format!("{}/shader_source.rs", &out_dir), &shader_source).unwrap();
    }
}
