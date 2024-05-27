use std::collections::HashMap;

pub fn Interpreter(instruct: Vec<Vec<String>>) {
    let mut int_vars: HashMap<i128, String> = HashMap::new();
    let mut float_vars: HashMap<f64, String> = HashMap::new();
    let mut str_vars: HashMap<String, String> = HashMap::new();
    let mut bool_vars: HashMap<bool, String> = HashMap::new();

    for i in instruct {
        if !i.is_empty() {
            if i[0] == "print" {
                if i.len() > 1 {
                    println!("{}", i[1]);
                } else {
                    eprintln!("Error: Missing argument for 'print' command");
                }
            }else if i[0] == "var" {
                if i.len() > 1 {
                    println!("new var");
                    if i[1] == "intvar" {
                        println!("INT");
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