use std::collections::HashMap;
use std::process::Command;

use crate::core;

///create awesome compiled package from uncompiled
fn compile_it(package: core::CodePackage) -> Result<core::TestablePackage, &'static str> {
    let extension = package
        .code_path
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .expect("doesn't have an extension")
        .to_string();
    println!("Extension: {}", extension);
    //needs rustc to compile this
    let mut map = HashMap::<&str, i32>::with_capacity(1);
    map.insert("rs", 0);
    map.insert("cpp", 1);

    if !map.contains_key(extension.as_str()) {
        return Err("Invalid Extension");
    }

    match map.get(extension.as_str()).unwrap() {
        0 => {
            let output = Command::new("rustc")
                .arg(package.code_path.as_str())
                .output()
                .expect("Failed to execute process rustc");
            println!(
                "rustc compilation:\nCodeFile: {}\nStatus: {}\nStdout: {}\nStderr: {}",
                package.code_path,
                output.status,
                String::from_utf8(output.stdout).expect("not valid utf8"),
                String::from_utf8(output.stderr).expect("not valid utf8"),
            );
            if !output.status.success() {
                return Err("rustc failed.");
            }
        }
        1 => {
            let output = Command::new("g++")
                .arg(package.code_path.as_str())
                .output()
                .expect("Failed to execute process g++");
            println!(
                "g++ compilation:\nCodeFile: {}\nStatus: {}\nStdout: {}\nStderr: {}",
                package.code_path,
                output.status,
                String::from_utf8(output.stdout).expect("not valid utf8"),
                String::from_utf8(output.stderr).expect("not valid utf8"),
            );
            if !output.status.success() {
                return Err("g++ failed.");
            }
        }
        _ => (),
    }

    let path = package
        .code_path
        .split_at(package.code_path.len() - extension.len() - 2)
        .0;

    Ok(core::TestablePackage {
        exe_path: path.to_string(),
        sha256: package.sha256,
        team_id: package.team_id,
        prob_num: package.prob_num,
    })
}

pub fn test_compile() {
    let code = core::CodePackage {
        code_path: "./.env/fileStation/lol.rs".to_string(),
        sha256: "hwklfdwsjflkds".to_string(),
        team_id: "plopco".to_string(),
        prob_num: 0,
    };
    compile_it(code).expect("Test failed");
}
