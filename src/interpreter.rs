use std::collections::HashMap;

pub fn Interpreter(instruct: Vec<Vec<String>>) {
    let mut int_vars: HashMap<String, i64> = HashMap::new();
    let mut float_vars: HashMap<String, f64> = HashMap::new();
    let mut str_vars: HashMap<String, String> = HashMap::new();
    let mut bool_vars: HashMap<String, bool> = HashMap::new();

    for i in instruct {
        if !i.is_empty() {
            if i[0] == "print" {
                if i.len() > 1 {
                    println!("{}", i[1]);
                } else {
                    eprintln!("Error: Missing argument");
                }
            }else if i[0] == "var" {
                if i.len() > 1 {
                    if i[1] == "intvar" {
                        let intvar: i64 = i[3].trim().parse().expect("");
                        int_vars.insert(i[2].to_string(), intvar);
                    }else if i[1] == "floatvar" {
                        println!("FLOAT");
                    }else if i[1] == "strvar" {
                        println!("STR");
                    }else if i[1] == "boolvar" {
                        println!("BOOL")
                    }
                }
            }
        }
    }
}