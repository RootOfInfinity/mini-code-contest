use std::fs;

#[derive(Debug)]
pub struct CodePackage {
    pub code_path: String,
    pub sha256: String,
    pub team_id: String,
    pub prob_num: u32,
}

#[derive(Debug)]
pub struct TestablePackage {
    pub exe_path: String,
    pub sha256: String,
    pub team_id: String,
    pub prob_num: u32,
    pub run_type: u32,
}

#[derive(Debug)]
pub struct FinalPackage {
    pub sha256: String,
    pub team_id: String,
    pub prob_num: u32,
    pub solved: bool,
}
