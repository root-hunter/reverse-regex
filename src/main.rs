use regex_syntax::parse;

mod engine;

pub fn main() {
    let pattern = r"Hello(!)? (World|Mondo|Mundo)";
    let hir = parse(pattern).unwrap();
    
    for _ in 0..10 {
        let result = engine::generate_string(hir.clone());
        println!("{}", result);
    }
}