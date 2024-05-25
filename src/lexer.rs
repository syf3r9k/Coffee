pub fn Lexer(code: String) {
    let mut interval = 0;
    if code.starts_with("case") {
        println!("NEW TOKEN VARIABLE");
        interval += 5;
        if code[interval..].starts_with("int") {
            println!("NEW TOKEN INTEGER");
            interval += 4;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);

            interval += name + 3;

            let mut code = &code[interval..];

            println!("{}", code);

        }else if code[interval..].starts_with("float") {
            println!("NEW TOKEN FLOAT");
            interval += 6;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);

            interval += name + 3;

            let mut code = &code[interval..];

            println!("{}", code);

        }else if code[interval..].starts_with("str") {
            println!("NEW TOKEN STR");
            interval += 4;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);

            interval += name + 3;

            let mut code = &code[interval..];

            println!("{}", code);

        }else if code[interval..].starts_with("bool") {
            println!("NEW TOKEN BOOL");
            interval += 5;
            let mut name = 0;
            for i in code[interval..].chars() {
                if i.to_string() != " " {
                    name += 1;
                } else {
                    break;
                }
            }
            println!("NEW TOKEN NAME: {}", &code[interval..interval + name]);

            interval += name + 3;

            let mut code = &code[interval..];

            println!("{}", code);
        }
    }
}