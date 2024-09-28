use regex_syntax::hir::{self, Hir, HirKind};
use rand::{self, Rng};

const MAX_ITERATIONS: u32 = 128;

pub fn generate_string(hir: Hir) -> String {
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
                    num = max;
                } else {
                    num = rand::thread_rng().gen_range(rep.min..max) as u32;
                }
            } else {
                num = rand::thread_rng().gen_range(rep.min..MAX_ITERATIONS) as u32;
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
                index = rand::thread_rng().gen_range(0..alt.len());
            }
            
            let choice = alt.get(index).unwrap().clone();
            
            return generate_string(choice);
        },
        HirKind::Concat(concats) => {
            let mut parts: Vec<String> = Vec::new();

            for h in concats {
                let hit = h.clone();
                parts.push(generate_string(hit));
            }
            
            return parts.join("").to_string();
        },
        HirKind::Class(class) => {
            match class {
                hir::Class::Unicode(class_unicode) => {
                    let ranges = class_unicode.ranges();
                    let mut index = 0;
                    
                    if ranges.len() > 0 {
                        index = rand::thread_rng().gen_range(0..ranges.len());
                    }

                    let class = ranges[index];

                    let char: char = rand::thread_rng().gen_range(class.start()..class.end());

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
            println!("OTHER");
            return String::from("Errrororoororor");
        }
    }
}