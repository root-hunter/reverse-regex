#[cfg(test)]
pub mod tests {
    use regex::Regex;
    use regex_syntax::parse;
    use reverse_regex::engine::generate;

    #[test]
    fn test_pattern_1() {
        let pattern = r"((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])\.){3}(25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])";
        let regex = Regex::new(pattern).unwrap();

        let hir = parse(pattern).unwrap();

        for _ in 0..100 {
            let result = generate(hir.clone());
            assert!(regex.is_match(result.as_str()));
        }
    }

    #[test]
    fn test_pattern_2() {
        let pattern = r"(lol|xd|dx){2,3}[0-9]{3,6}(anto|bob|john|mike)@(gmail|outlook|hotmail)[0-9]{1,10}\.(it|com|org|nas)";
        let regex = Regex::new(pattern).unwrap();

        let hir = parse(pattern).unwrap();
        
        for _ in 0..100 {
            let result = generate(hir.clone());
            assert!(regex.is_match(result.as_str()));
        }
    }

    #[test]
    fn test_pattern_3() {
        let pattern = r"(GB)?(GD[0-9]{3}|HA[0-9]{3}|[0-9]{9}|[0-9]{12})";
        let regex = Regex::new(pattern).unwrap();

        let hir = parse(pattern).unwrap();
        
        for _ in 0..100 {
            let result = generate(hir.clone());
            assert!(regex.is_match(result.as_str()));
        }
    }
}