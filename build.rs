use std::env::var;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

fn run_cmake(source_dir: &str) {
    let sources = [
        "source/dy.cc",
        "public/dy.h",
        "private/dy.p.hh",
        "CMakeLists.txt",
    ];
    for src in sources.iter() {
        println!("cargo:rerun-if-changed={}/{}", source_dir, src);
    }

    let install_dir = cmake::build(source_dir);
    println!(
        "cargo:rustc-link-search=native={}/bin",
        install_dir.display()
    );
    println!("cargo:rustc-link-lib=static={}", source_dir);
    println!("cargo:rustc-link-lib=dylib={}", source_dir);
}

fn generate_bindings(header_path: &str) -> Result<()> {
    // generate bindings
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out = Path::new("src/bindings.rs");
    let mut file = File::create(&out)?;

    // write to file
    let error_types = [
        "non_camel_case_types",
        "non_snake_case",
        "non_upper_case_globals",
        "dead_code",
    ];
    for error_type in error_types.iter() {
        writeln!(file, "#![allow({})]", error_type)?;
    }
    write!(file, "\n{}", bindings.to_string())?;
    Ok(())
}

fn main() -> Result<()> {
    // Set profile as an environment variable: used to build tests/import_test
    println!("cargo:rustc-env=PROFILE={}", var("PROFILE").unwrap());
    run_cmake("dy");
    generate_bindings("dy/public/dy.h")
}
