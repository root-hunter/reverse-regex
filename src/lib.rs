pub mod engine;

#[cfg(feature = "web")]
pub mod web {
    use wasm_bindgen::prelude::*;
    use regex_syntax::parse;

    use crate::engine::{self, EngineConfig, ENGINE_DEFAULT_CONFIG};

    #[wasm_bindgen]
    pub fn generate(pattern: String, configs: Option<EngineConfig>) -> String {
        let _configs: EngineConfig;

        if let Some(configs) = configs {
            _configs = configs;
        } else {
            _configs = ENGINE_DEFAULT_CONFIG;
        };

        let mut pattern = pattern;

        if _configs.force_decimal {
            pattern = pattern.replace("\\d", "[[:digit:]]");
        }

        if _configs.force_alphanumeric {
            pattern = pattern.replace("\\w", "[a-zA-Z0-9_]");
        }

        let hir = parse(&pattern).unwrap();
        return engine::generate_string(hir, &_configs);
    }
}

pub enum TestPatternEnum {
    IPV4,
    IPV6,
    EMAIL,
}

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

    use crate::{engine, TestPatternEnum};

    pub fn _test_pattern(ttype: TestPatternEnum) {
        let pattern = super::get_test_pattern(ttype);
        let pattern = pattern.as_str();

        let regex = Regex::new(pattern).unwrap();

        let hir = parse(pattern).unwrap();
        let configs = engine::ENGINE_DEFAULT_CONFIG;

        for _ in 0..100 {
            let result = engine::generate_string(hir.clone(), &configs);
            assert!(regex.is_match(result.as_str()));
        }
    }

    #[test]
    pub fn test_pattern_ipv4() {
        _test_pattern(TestPatternEnum::IPV4);
    }

    #[test]
    pub fn test_pattern_ipv6() {
        _test_pattern(TestPatternEnum::IPV6);
    }

    #[test]
    pub fn test_pattern_email() {
        _test_pattern(TestPatternEnum::EMAIL);
    }

    #[cfg(feature = "web")]
    use wasm_bindgen_test::*;

    #[cfg(feature = "web")]
    #[wasm_bindgen_test]
    pub fn test_pattern_ipv4_wasm() {
        _test_pattern(TestPatternEnum::IPV4);
    }

    #[cfg(feature = "web")]
    #[wasm_bindgen_test]
    pub fn test_pattern_ipv6_wasm() {
        _test_pattern(TestPatternEnum::IPV6);
    }

    #[cfg(feature = "web")]
    #[wasm_bindgen_test]
    pub fn test_pattern_email_wasm() {
        _test_pattern(TestPatternEnum::EMAIL);
    }
}
