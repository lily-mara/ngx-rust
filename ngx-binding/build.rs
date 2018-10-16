extern crate bindgen;

use std::env;
use std::io;
use std::process;
use std::process::Command;
use std::process::Output;

#[derive(Debug)]
enum MakeError {
    IoError(io::Error),
    NonZeroExitCode(process::ExitStatus),
}

impl From<io::Error> for MakeError {
    fn from(e: io::Error) -> MakeError {
        MakeError::IoError(e)
    }
}

#[cfg(target_os = "macos")]
const NGIX_DIR: &str = "./nginx/nginx-darwin";

#[cfg(target_os = "linux")]
const NGIX_DIR: &str = "./nginx/nginx-linux";

const NGX_VAR_REGEX: &str = "[nN][gG][xX]_.*";

// perform make with argument
fn make(arg: &str) -> Result<Output, MakeError> {
    let current_path = env::current_dir().unwrap();
    let path_name = format!("{}", current_path.display());
    println!("executing make command at {}", path_name);
    let output = Command::new("/usr/bin/make")
        .args(&[arg])
        .current_dir(path_name)
        .output()?;

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(MakeError::NonZeroExitCode(output.status));
    }

    return Ok(output);
}

fn configure() -> Result<Output, MakeError> {
    make("nginx-configure")
}

fn generate_binding() {
    let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header("wrapper.h")
    .layout_tests(false)
    .clang_arg(format!("-I{}/src/core",NGIX_DIR))
    .clang_arg(format!("-I{}/src/event",NGIX_DIR))
    .clang_arg(format!("-I{}/src/event/modules",NGIX_DIR))
    .clang_arg(format!("-I{}/src/os/unix",NGIX_DIR))
    .clang_arg(format!("-I{}/objs",NGIX_DIR))
    .clang_arg(format!("-I{}/src/http",NGIX_DIR))
    .clang_arg(format!("-I{}/src/http/modules",NGIX_DIR))
    // Whitelist the nginx types
    .whitelist_var(NGX_VAR_REGEX)
    .whitelist_type(NGX_VAR_REGEX)
    .whitelist_function(NGX_VAR_REGEX)
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    configure().expect("failed to configure nginx");
    generate_binding();
}
