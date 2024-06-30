use std::env;
use std::path::PathBuf;

fn main() {
    // cc::Build::new()
    //     .files(fs::read_dir("codegen").unwrap())
    //     .flag_if_supported("-std=c++11")
    //     .compile("codegen_classes");

    println!("cargo:rerun-if-changed=codegen/classes.hpp");

    let bindings = bindgen::Builder::default()
        .header("codegen/classes.hpp")
        .header("codegen/idadefs.hpp")
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
