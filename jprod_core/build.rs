use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[cfg(feature = "develop")]
fn trim_whitespace(s: &str) -> String {
    s.to_owned()
}

#[cfg(not(feature = "develop"))]
fn trim_whitespace(s: &str) -> String {

    let mut temp = String::new();

    for line in s.lines() {
        if let Some(beginning) = line.split("//").next() {
            temp.push_str(beginning);
            temp.push('\n');
        }
    }

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

fn to_camel_case(s: &str) -> String {
    
    let mut should_uppercase = true;
    let mut res = String::new();
    
    for c in s.chars() {
        if c == '_' {
            should_uppercase = true;
        } else {
            if should_uppercase {
                res.push_str(&c.to_uppercase().collect::<String>());
                should_uppercase = false;
            } else {
                res.push(c);
            }
        }
    }

    res
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}


fn write_file<P: AsRef<Path>>(path: P, data: &str) -> Result<(), Error> {
    let mut f = try!(File::create(path));
    try!(f.write_all(data.as_bytes()));
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

    let mut shaders = HashMap::new();

    if let Ok(rd) = fs::read_dir("src/shaders") {
        for entry in rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file()) {

            let name = to_camel_case(entry.file_stem().unwrap().to_str().unwrap());
            let extension = entry.extension().unwrap().to_str().unwrap();

            if extension == "vert" || extension == "frag" {

                let shader = shaders.entry(name).or_insert(ShaderData::new());

                if extension == "vert" {
                    shader.vertex_source = Some(trim_whitespace(&read_file(&entry).unwrap()));
                } else if extension == "frag" {
                    shader.fragment_source = Some(trim_whitespace(&read_file(&entry).unwrap()));
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

        shader_source.push_str("#[inline]\npub fn get_shader_source(id: ShaderId) -> ShaderSource { match id {");

        for (name, shader) in shaders {

            if let (&Some(ref vertex), &Some(ref fragment)) = (&shader.vertex_source, &shader.fragment_source) {
                
                shader_source.push_str(
                        &format!("ShaderId::{} => ShaderSource {{ vertex_source: {}, fragment_source: {} }},",
                                name,
                                format!(r###"br##"{}"##"###, vertex),
                                format!(r###"br##"{}"##"###, fragment)));
            }
        }

        shader_source.push_str("}}");

        write_file(&format!("{}/shader_source.rs", out_dir), &shader_source).unwrap();
    }
}