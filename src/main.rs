pub mod compiler;
pub mod core;
pub mod tester;

fn main() {
    println!("Hello, world!");
    compiler::test_compile();
}
