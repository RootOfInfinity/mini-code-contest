use std::fs;

#[derive(Debug)]
pub struct CodePackage {
    pub codePath: String,
    pub sha256: String,
    pub teamID: String,
    pub prob_num: String,
}

#[derive(Debug)]
pub struct TestablePackage {
    pub exePath: String,
    pub sha256: String,
    pub teamID: String,
    pub prob_num: u32,
}

#[derive(Debug)]
pub struct FinalPackage {
    pub sha256: String,
    pub teamID: String,
    pub prob_num: u32,
    pub solved: bool,
}
