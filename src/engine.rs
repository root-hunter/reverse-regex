use regex_syntax::hir::{self, Hir, HirKind};
use rand::{self, Rng};

const MAX_ITERATIONS: u32 = 128;

pub struct EngineConfig {

}

pub fn generate_string(hir: Hir) -> String {
    let mut rng = rand::thread_rng();

    match hir.kind() {
        HirKind::Capture(capture) => {
            let capture = capture.clone();
            let sub = capture.sub;

            return generate_string(*sub);
        },
        HirKind::Repetition(rep) => {
            let mut parts: Vec<String> = Vec::new();
            
            let num: u32;
            
            if let Some(max) = rep.max {
                if rep.min == max {
                    num = max + 1;
                } else {
                    num = rng.gen_range(rep.min..(max + 1)) as u32 ;
                }
            } else {
                num = rng.gen_range(rep.min..MAX_ITERATIONS) as u32;
            };

            for _ in 0..num {
                let sub = rep.clone().sub;
                parts.push(generate_string(*sub));
            }

            return parts.join("").to_string();
        },
        HirKind::Literal(lit) => {
            return String::from_utf8(lit.0.to_vec()).unwrap();
        },
        HirKind::Alternation(alt) => {
            let mut index = 0;
            if alt.len() > 0 {
                index = rng.gen_range(0..alt.len());
            }
            
            let choice = alt.get(index).unwrap().clone();
            
            return generate_string(choice);
        },
        HirKind::Concat(concats) => {
            let mut parts: Vec<String> = Vec::new();

            for part in concats {
                let hir = part.clone();
                parts.push(generate_string(hir));
            }
            
            return parts.join("").to_string();
        },
        HirKind::Class(class) => {
            match class {
                hir::Class::Unicode(class_unicode) => {
                    let ranges = class_unicode.ranges();
                    let mut index = 0;
                    
                    if ranges.len() > 0 {
                        index = rng.gen_range(0..ranges.len());
                    }

                    let class = ranges[index];

                    let first_char = class.start() as u32;
                    let first_char = char::from_u32(first_char).unwrap();
                    
                    let last_char = class.end() as u32 + 1;
                    let last_char = char::from_u32(last_char).unwrap();

                    let char: char = rng.gen_range(first_char..last_char);

                    return char.to_string();
                },
                hir::Class::Bytes(_) => {
                    return "*".to_string();
                },
            };
        },
        HirKind::Empty => {
            return "".to_string();
        },
        _ => {
            return String::from("*ERROR*");
        }
    }
}