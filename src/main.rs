use regex_syntax::parse;

mod engine;

pub fn main() {
    let pattern = r"((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])\.){3}(25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])";
    let hir = parse(pattern).unwrap();
    
    for _ in 0..10 {
        let result = engine::generate_string(hir.clone());
        println!("{}", result);
    }
}