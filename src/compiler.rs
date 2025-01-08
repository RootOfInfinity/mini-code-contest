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
    map.insert("java", 2);
    map.insert("py", 3);

    if !map.contains_key(extension.as_str()) {
        return Err("Invalid Extension");
    }

    match map.get(extension.as_str()).unwrap() {
        0 => {
            let output = Command::new("rustc")
                .args([
                    package.code_path.as_str(),
                    "-o",
                    package
                        .code_path
                        .split_at(package.code_path.len() - extension.len() - 1)
                        .0,
                ])
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
                .args([
                    package.code_path.as_str(),
                    "-o",
                    package
                        .code_path
                        .split_at(package.code_path.len() - extension.len() - 1)
                        .0,
                ])
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
        2 => {
            println!("java will be compiled at just in time");
        }
        3 => {
            println!("python is interpreted and will not be compiled");
        }
        _ => (),
    }

    let path = {
        if map.get(&extension.as_str()).unwrap() < &2 {
            package
                .code_path
                .split_at(package.code_path.len() - extension.len() - 1)
                .0
        } else {
            package.code_path.as_str()
        }
    };

    Ok(core::TestablePackage {
        exe_path: path.to_string(),
        sha256: package.sha256,
        team_id: package.team_id,
        prob_num: package.prob_num,
        run_type: *map.get(extension.as_str()).unwrap() as u32,
    })
}

// unit tests only pass if files (amazing.py, cool.cpp, lol.rs, vert.java) exist
#[test]
fn test_compile_rust() {
    let code = core::CodePackage {
        code_path: "./.env/fileStation/lol.rs".to_string(),
        sha256: "hwklfdwsjflkds".to_string(),
        team_id: "amazing".to_string(),
        prob_num: 0,
    };
    compile_it(code).expect("Test failed");
}

#[test]
fn test_compile_cpp() {
    let code = core::CodePackage {
        code_path: "./.env/fileStation/cool.cpp".to_string(),
        sha256: "hwrhejwkjrdfds".to_string(),
        team_id: "awesome".to_string(),
        prob_num: 0,
    };
    compile_it(code).expect("Test failed");
}

#[test]
fn test_compile_java() {
    let code = core::CodePackage {
        code_path: "./.env/fileStation/vert.java".to_string(),
        sha256: "kjureeiwrdnj".to_string(),
        team_id: "cool".to_string(),
        prob_num: 0,
    };
    compile_it(code).expect("Test failed");
}

#[test]
fn test_compile_python() {
    let code = core::CodePackage {
        code_path: "./.env/fileStation/amazing.py".to_string(),
        sha256: "euiwodnfkvcx".to_string(),
        team_id: "pythonbros".to_string(),
        prob_num: 0,
    };
    compile_it(code).expect("Test failed");
}
