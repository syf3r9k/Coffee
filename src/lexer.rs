pub fn Lexer(code: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut interval = 0;
    if code.starts_with("case") {
        //println!("NEW TOKEN VARIABLE");
        tokens.push("var".to_string());
        interval += 5;
        if code[interval..].starts_with("int") {
            //println!("NEW TOKEN INTEGER");
            tokens.push("intvar".to_string());
            interval += 4;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            //println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);
            let n = &code[interval..interval + name];
            tokens.push(n.to_string());

            interval += name + 3;

            let mut code = &code[interval..];
            let code = code.replace(" ", "");

            for i in code.chars() {
                if i.to_string() == "1" ||
                    i.to_string() == "2" ||
                    i.to_string() == "3" ||
                    i.to_string() == "4" ||
                    i.to_string() == "5" ||
                    i.to_string() == "6" ||
                    i.to_string() == "7" ||
                    i.to_string() == "8" ||
                    i.to_string() == "9" ||
                    i.to_string() == "0" {
                    //println!("NEW TOKEN NUMBER: {}", i);
                    tokens.push(i.to_string());
                } else if i.to_string() == "+" ||
                    i.to_string() == "-" ||
                    i.to_string() == "*" ||
                    i.to_string() == "/" ||
                    i.to_string() == "(" ||
                    i.to_string() == ")" {
                    //println!("NEW TOKEN OPERATOR: {}", i);
                    tokens.push("operator".to_string());
                }
            }
        }
        else if code[interval..].starts_with("float") {
            //println!("NEW TOKEN FLOAT");
            tokens.push("floatvar".to_string());
            interval += 6;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            //println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);
            tokens.push("namevar".to_string());
            interval += name + 3;

            let mut code = &code[interval..];
            for i in code.chars() {
                if i.to_string() == "1" ||
                    i.to_string() == "2" ||
                    i.to_string() == "3" ||
                    i.to_string() == "4" ||
                    i.to_string() == "5" ||
                    i.to_string() == "6" ||
                    i.to_string() == "7" ||
                    i.to_string() == "8" ||
                    i.to_string() == "9" ||
                    i.to_string() == "0" {
                    //println!("NEW TOKEN NUMBER: {}", i);
                    tokens.push("number".to_string());
                } else if i.to_string() == "+" ||
                    i.to_string() == "-" ||
                    i.to_string() == "*" ||
                    i.to_string() == "/" ||
                    i.to_string() == "(" ||
                    i.to_string() == ")" ||
                    i.to_string() == "." {
                    //println!("NEW TOKEN OPERATOR: {}", i);
                    tokens.push("operator".to_string());
                }
            }
        }
        else if code[interval..].starts_with("str") {
            //println!("NEW TOKEN STR");
            tokens.push("strvar".to_string());
            interval += 4;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            //println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);
            tokens.push("namevar".to_string());
            interval += name + 3;
            let mut code = &code[interval..];
            //println!("NEW TOKEN STRING: {}", code.replace("'", ""));
            tokens.push(code.replace("'", "").to_string());
        }
        else if code[interval..].starts_with("bool") {
            //println!("NEW TOKEN BOOL");
            tokens.push("boolvar".to_string());
            interval += 5;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            //println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);
            tokens.push("namevar".to_string());
            interval += name + 3;
            let mut code = &code[interval..];
            //println!("{}", code);
            tokens.push(code.to_string());
        }
    }
    if code.starts_with("pr!") {
        tokens.push("print".to_string());
        let code = &code[3..];
        let code = code.replace("('", "");
        let code = code.replace("')", "");
        tokens.push(code.to_string());
    }

    tokens
}