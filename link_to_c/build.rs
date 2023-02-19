use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new()
        .include("RustTestStatic/RustTestStatic")
        .file("RustTestStatic/RustTestStatic/RustTestStatic.cpp")
        .cpp(true)
        .compile("RustTestStatic");

    Ok(())
}
