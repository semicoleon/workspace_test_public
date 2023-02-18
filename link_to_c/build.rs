use std::error::Error;

fn main()-> Result<(), Box<dyn Error>> {
    // Get the proper directory for this based on the build profile
    let profile = std::env::var("PROFILE").unwrap();
    let subdir = match profile.as_str() {
        "debug" => "Debug",
        "release" => "Release",
        _ => panic!("Unknown cargo profile:"),
    };
    let cur_path = std::env::current_dir()?;
    println!("cargo:rustc-link-search={}\\RustTestStatic\\x64\\{subdir}", cur_path.display());

    Ok(())
}
