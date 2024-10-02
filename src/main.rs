mod engine;
use regex_syntax::parse;

pub fn main() {
    let pattern = r"Hello (world|mundo|moon)";
    let pattern = pattern.replace("\\d", "[[:digit:]]");
    let hir = parse(pattern.as_str()).unwrap();
    let configs = engine::ENGINE_DEFAULT_CONFIG;

    for _ in 0..10 {
        let result = engine::generate_string(hir.clone(), &configs);
        println!("{}", result);
    }
}