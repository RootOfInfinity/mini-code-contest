extern crate assert_cmd;
extern crate json;

use std::fs;
use std::sync::Arc;

use assert_cmd::Command;
use json::JsonValue;

use crate::core;

//plan is to create a json format that has differing catagories and tests
#[derive(Clone)]
pub struct Test {
    input: Arc<str>,
    output: Arc<str>,
}

pub struct TestSet {
    name: Arc<str>,
    test_num: u32,
    description: Arc<str>,
    tests: Arc<[Test]>,
}

impl TestSet {
    fn from_json(testj: JsonValue) -> Self {
        let test_len: usize = testj["testcases"][0]
            .as_usize()
            .expect("testcase length not usize");
        let mut cont = Vec::with_capacity(test_len);
        for i in 1..=test_len {
            let ntest = Test {
                input: Arc::from(testj["testcases"][i]["input"].as_str().unwrap()),
                output: Arc::from(testj["testcases"][i]["output"].as_str().unwrap()),
            };
            cont.push(ntest);
        }
        Self {
            name: Arc::from(testj["name"].as_str().unwrap()),
            test_num: testj["test_num"].as_u32().unwrap(),
            description: Arc::from(testj["description"].as_str().unwrap()),
            tests: cont.into(),
        }
    }
}

fn test_against_testcases(
    tests: Arc<TestSet>,
    package: core::TestablePackage,
) -> core::FinalPackage {
    let mut right = 0;
    if tests.test_num != package.prob_num {
        //if it isnt the right testset
        return core::FinalPackage {
            sha256: package.sha256,
            team_id: package.team_id,
            prob_num: package.prob_num,
            solved: false,
        };
    }
    let val = Arc::clone(&tests);
    for test in val.tests.iter().cloned() {
        //essentially, this will panic if it don work, we need to fix allat.
        let mut cmd = Command::new(&package.exe_path);
        let assert = cmd
            .timeout(std::time::Duration::from_secs(2))
            .write_stdin(&*test.input)
            .assert();
        let output: String = (*test.output).to_string();
        assert.stdout(output);
    }
    todo!()
    //figure out how to make this concurrent and stop after an amount of time
}

///look for files in test folder "./.env/fileStation" TEMPORARY
pub fn look_for_executables() -> Option<Vec<core::TestablePackage>> {
    let paths = fs::read_dir("./.env/fileStation").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

    //TODO get er done!
    None
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
