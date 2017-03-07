extern crate kernel32;
extern crate walkdir;
extern crate winapi;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use walkdir::WalkDir;
use winapi::fileapi::WIN32_FILE_ATTRIBUTE_DATA;
use winapi::minwinbase::GetFileExInfoStandard;
use winapi::minwindef::FILETIME;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

#[cfg(feature = "develop")]
fn trim_whitespace(s: &str) -> String {
    s.to_owned()
}

#[cfg(not(feature = "develop"))]
fn trim_whitespace(s: &str) -> String {

    let mut temp = s.to_owned();

    let matches = &[
        ("\r\n", "\n"),
        ("\n\n", "\n"),

        ("  ", " "),

        ("\n{", "{"),
        ("{\n", "{"),
        (" {", "{"),
        ("{ ", "{"),

        ("\n}", "}"),
        ("}\n", "}"),
        (" }", "}"),
        ("} ", "}"),

        ("\n)", ")"),
        (")\n", ")"),
        (" )", ")"),
        (") ", ")"),

        ("\n(", "("),
        ("(\n", "("),
        (" (", "("),
        ("( ", "("),

        ("\n;", ";"),
        (";\n", ";"),
        (" ;", ";"),
        ("; ", ";"),

        ("\n,", ","),
        (",\n", ","),
        (" ,", ","),
        (", ", ","),

        ("\n+", "+"),
        ("+\n", "+"),
        (" +", "+"),
        ("+ ", "+"),

        ("\n-", "-"),
        ("-\n", "-"),
        (" -", "-"),
        ("- ", "-"),

        ("\n*", "*"),
        ("*\n", "*"),
        (" *", "*"),
        ("* ", "*"),


        ("\n/", "/"),
        ("/\n", "/"),
        (" /", "/"),
        ("/ ", "/"),

        ("\n%", "%"),
        ("%\n", "%"),
        (" %", "%"),
        ("% ", "%"),

        ("\n=", "="),
        ("=\n", "="),
        (" =", "="),
        ("= ", "="),
    ];

    loop {

        let begin_len = temp.len();

        for &(pattern, value) in matches.iter() {

            loop {

                let old_len = temp.len();

                temp = temp.replace(pattern, value);

                if temp.len() == old_len {
                    break;
                }
            }
        }

        if temp.len() == begin_len {
            break;
        }
    }

    temp
}

fn write_file<P: AsRef<Path>>(path: P, data: &str) -> Result<(), Error> {
    let mut f = try!(File::create(path));
    try!(f.write_all(data.as_bytes()));
    Ok(())
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Filetime {
    low: u32,
    high: u32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct ShaderData {
    vertex_source: Option<String>,
    vertex_path: Option<String>,
    vertex_filetime: Option<Filetime>,
    fragment_source: Option<String>,
    fragment_path: Option<String>,
    fragment_filetime: Option<Filetime>,
}

impl ShaderData {
    fn new() -> ShaderData {
        ShaderData {
            vertex_source: None,
            vertex_path: None,
            vertex_filetime: None,
            fragment_source: None,
            fragment_path: None,
            fragment_filetime: None,
        }
    }
}

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut shaders = HashMap::new();

    for entry in WalkDir::new("src/shaders").into_iter().filter_map(|e| e.ok()) {

        if entry.metadata().unwrap().is_file() {

            let name = uppercase_first_letter(entry.path().file_stem().unwrap().to_str().unwrap());
            let extension = entry.path().extension().unwrap().to_str().unwrap();

            if extension == "vert" || extension == "frag" {

                let shader = shaders.entry(name).or_insert(ShaderData::new());

                let wide_path: Vec<u16> = entry.path().as_os_str().encode_wide().chain(once(0)).collect();

                let filetime = unsafe {

                    let mut data = WIN32_FILE_ATTRIBUTE_DATA {
                        dwFileAttributes: 0,
                        ftCreationTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
                        ftLastAccessTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
                        ftLastWriteTime: FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 },
                        nFileSizeHigh: 0,
                        nFileSizeLow: 0,
                    };

                    kernel32::GetFileAttributesExW(wide_path.as_ptr(), GetFileExInfoStandard, &mut data as *mut WIN32_FILE_ATTRIBUTE_DATA as *mut _);

                    data.ftLastWriteTime
                };         

                if extension == "vert" {
                    shader.vertex_source = Some(trim_whitespace(&read_file(entry.path()).unwrap()));
                    shader.vertex_path = Some(entry.path().canonicalize().unwrap().to_str().unwrap().to_string());
                    shader.vertex_filetime = Some(Filetime { low: filetime.dwLowDateTime, high: filetime.dwHighDateTime });
                } else if extension == "frag" {
                    shader.fragment_source = Some(trim_whitespace(&read_file(entry.path()).unwrap()));
                    shader.fragment_path = Some(entry.path().canonicalize().unwrap().to_str().unwrap().to_string());
                    shader.fragment_filetime = Some(Filetime { low: filetime.dwLowDateTime, high: filetime.dwHighDateTime });
                }
            }
        }
    }

    {
        let mut shader_ids = String::new();

        shader_ids.push_str("pub enum ShaderId {\n");

        for keys in shaders.keys() {
            shader_ids.push_str(&format!("{},", keys));
            shader_ids.push_str("\n");
        }

        shader_ids.push_str("}\n");

        write_file(&format!("{}/shader_ids.rs", out_dir), &shader_ids).unwrap();
    }

    {
        let mut shader_source = String::new();

        shader_source.push_str("pub fn get_shader_source(id: ShaderId) -> ShaderSource { match id {");

        for (name, shader) in shaders {

            if let (&Some(ref vertex), &Some(ref fragment), &Some(ref vertex_path), &Some(ref fragment_path), &Some(ref vertex_filetime), &Some(ref fragment_filetime)) =
                    (&shader.vertex_source, &shader.fragment_source, &shader.vertex_path, &shader.fragment_path, &shader.vertex_filetime, &shader.fragment_filetime) {
                
                shader_source.push_str(
                        &format!("ShaderId::{} => ShaderSource {{ vertex_source: {}, vertex_path: {}, vertex_filetime: Filetime {{ low_datetime: {}, high_datetime: {} }}, fragment_source: {}, fragment_path: {}, fragment_filetime: Filetime {{ low_datetime: {}, high_datetime: {} }} }},",
                                name,
                                format!(r###"br##"{}"##"###, vertex),
                                format!(r###"b"{}\0""###, vertex_path.replace("\\", "/")),
                                vertex_filetime.low,
                                vertex_filetime.high,
                                format!(r###"br##"{}"##"###, fragment),
                                format!(r###"b"{}\0""###, fragment_path.replace("\\", "/")),
                                fragment_filetime.low,
                                fragment_filetime.high));
            }
        }

        shader_source.push_str("}}");

        write_file(&format!("{}/shader_source.rs", out_dir), &shader_source).unwrap();
    }
}