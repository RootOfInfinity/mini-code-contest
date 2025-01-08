extern crate assert_cmd;

use std::fs;
use std::fs::File;

use assert_cmd::Command;

use crate::core;

///look for files in test folder "./.env/fileStation" TEMPORARY
pub fn look_for_executables() -> Option<Vec<core::TestablePackage>> {
    let paths = fs::read_dir("./.env/fileStation").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

    //TODO get er done!
    None
}

pub fn test_create_executable() -> Result<core::TestablePackage, String> {
    let sha256id: String =
        String::from("c8e0583694bb1e0188dbe28fe0d65ac1130723c55f968b6262b906c147f72549");
    let teamid: String = "lol".to_string();
    let num_problem: u32 = 0u32;

    todo!()
}

///run against testcases
pub fn testit(package: core::TestablePackage) -> core::FinalPackage {
    if package.prob_num == 0u32 {
        let mut cmd = Command::new("./.env/fileStation/hello");
        let assert = cmd.write_stdin("32").assert();
        assert.failure().stdout("Hello, world!\n");
    }

    todo!()
}

// the rest are helper functions

// unit tests

// #[test]
// fn test1() {
//     look_for_executables();
// }

// #[test]
// fn testHello() {
//     let mut cmd = Command::new("./.env/fileStation/hello");
//     let assert = cmd
//         .write_stdin("32")
//         .assert();
//     assert
//         .success()
//         .stdout("Hello, world!\n");
// }
