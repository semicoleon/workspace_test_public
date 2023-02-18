# workspace_test_public
Repository to demonstrate basic Rust workspace functionality, including linking C/C++ libraries from within it.

**Note: Does not function correctly as of this commit.  Requires build.rs in the executable directory, which seems incorrect.**

# Build Requirements
- Latest Rust compiler - [Official Site](https://www.rust-lang.org/)
- Visual Studio 2019 or higher - [Microsoft](https://visualstudio.microsoft.com/downloads/) - Community should work
- Windows 10 or higher

All instructions below assume that both Visual Studio and Rust are installed and function correctly.

# Build Instructions

1. Open .\link_to_c\RustTestStatic\RustTestStatic.sln in Visual Studio.  It should be on the x64/Debug profile by default.  Build the solution.  Then change the profile to "Release" and build that as well.
  - Confirm that .\link_to_c\RustTestStatic\x64\Debug\RustTestStatic.lib and the same in the Release folder exist.  You can try out ConsoleTest.exe in each of those directories too to confirm that it built OK.
2. Go to the root directory of the repository and type `cargo build` to build the entire workspace.  Type `cargo run` and confirm that the program functions
3. Also in the root directory, type `cargo build --release`.  This will fail on the master branch, but succeed on the "with_console_buildrs" branch

