// build.rs
use std::env;

use rustantic;

fn main() {
    println!("cargo:warning=Start Rustantic Build Script");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pydantic_module = "generated";
    let module_name = "rustantic_test";
    let py_path = format!("{}/{}/{}", manifest_dir, module_name, pydantic_module);
    let models_package_name = format!("{}.{}", module_name, pydantic_module);
    rustantic::generate(module_name, &py_path, &manifest_dir, &models_package_name);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
