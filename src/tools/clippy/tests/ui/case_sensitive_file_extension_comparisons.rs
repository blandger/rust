#![warn(clippy::case_sensitive_file_extension_comparisons)]
#![allow(clippy::unnecessary_map_or)]

use std::string::String;

struct TestStruct;

impl TestStruct {
    fn ends_with(self, _arg: &str) {}
}

#[allow(dead_code)]
fn is_rust_file(filename: &str) -> bool {
    filename.ends_with(".rs")
    //~^ case_sensitive_file_extension_comparisons
}

fn main() {
    // std::string::String and &str should trigger the lint failure with .ext12
    let _ = String::new().ends_with(".ext12");
    //~^ case_sensitive_file_extension_comparisons
    let _ = "str".ends_with(".ext12");
    //~^ case_sensitive_file_extension_comparisons

    // The fixup should preserve the indentation level
    {
        let _ = "str".ends_with(".ext12");
        //~^ case_sensitive_file_extension_comparisons
    }

    // The test struct should not trigger the lint failure with .ext12
    TestStruct {}.ends_with(".ext12");

    // std::string::String and &str should trigger the lint failure with .EXT12
    let _ = String::new().ends_with(".EXT12");
    //~^ case_sensitive_file_extension_comparisons
    let _ = "str".ends_with(".EXT12");
    //~^ case_sensitive_file_extension_comparisons

    // Should not trigger the lint failure because of the calls to to_lowercase and to_uppercase
    let _ = String::new().to_lowercase().ends_with(".EXT12");
    let _ = String::new().to_uppercase().ends_with(".EXT12");

    // The test struct should not trigger the lint failure with .EXT12
    TestStruct {}.ends_with(".EXT12");

    // Should not trigger the lint failure with .eXT12
    let _ = String::new().ends_with(".eXT12");
    let _ = "str".ends_with(".eXT12");
    TestStruct {}.ends_with(".eXT12");

    // Should not trigger the lint failure with .EXT123 (too long)
    let _ = String::new().ends_with(".EXT123");
    let _ = "str".ends_with(".EXT123");
    TestStruct {}.ends_with(".EXT123");

    // Shouldn't fail if it doesn't start with a dot
    let _ = String::new().ends_with("a.ext");
    let _ = "str".ends_with("a.extA");
    TestStruct {}.ends_with("a.ext");

    // Shouldn't fail if the extension has no ascii letter
    let _ = String::new().ends_with(".123");
    let _ = "str".ends_with(".123");
    TestStruct {}.ends_with(".123");
}
