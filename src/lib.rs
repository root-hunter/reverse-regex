use regex_syntax::parse;
use wasm_bindgen::prelude::*;

pub mod engine;

#[wasm_bindgen]
pub fn generate(pattern: String) -> String{
    let hir = parse(pattern.as_str()).unwrap();
    return engine::generate_string(hir);
}

#[wasm_bindgen]
pub fn hello(pattern: String) -> String{
    return format!("LOLO: {}", pattern);
}

#[wasm_bindgen]
pub enum TestPatternEnum {
    IPV4,
    IPV6,
    EMAIL,
}

#[wasm_bindgen]
pub fn get_test_pattern(ttype: TestPatternEnum) -> String {
    return match ttype {
        TestPatternEnum::IPV4 => r"((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])\.){3}(25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])",
        TestPatternEnum::IPV6 => r"((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])\.){3}(25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])",
        TestPatternEnum::EMAIL => r"(local|dev|test){0,3}(anto|bob|john|mike|frank|danny)[0-9]{1,10}@(gmail|outlook|hotmail|web\.proton)\.(it|com|org|uk|fr|de|lu|ir)",
    }.to_string();
}

#[cfg(test)]
pub mod tests {
    use regex::Regex;
    use regex_syntax::parse;
    use wasm_bindgen_test::*;

    use crate::{engine, TestPatternEnum};

    pub fn _test_pattern(ttype: TestPatternEnum) {
        let pattern = super::get_test_pattern(ttype);
        let pattern = pattern.as_str();

        let regex = Regex::new(pattern).unwrap();

        let hir = parse(pattern).unwrap();

        for _ in 0..100 {
            let result = engine::generate_string(hir.clone());
            assert!(regex.is_match(result.as_str()));
        }
    }

    #[test]
    #[wasm_bindgen_test]
    pub fn test_pattern_ipv4() {
        _test_pattern(TestPatternEnum::IPV4);
    }

    #[test]
    #[wasm_bindgen_test]
    pub fn test_pattern_ipv6() {
        _test_pattern(TestPatternEnum::IPV6);
    }

    #[test]
    #[wasm_bindgen_test]
    pub fn test_pattern_email() {
        _test_pattern(TestPatternEnum::EMAIL);
    }
}