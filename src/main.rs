use std::io;

struct Context{
    stack: Vec<i32>,
}

impl Context {
    fn new() -> Context {
        Context {
            stack : Vec::new(),
        }
    }

    fn run(&mut self, tokens : Vec<Token>) -> Result<(), &'static str> {
        macro_rules! terme {
            () => {
                match self.stack.pop() {
                    Some(value) => value,
                    None => return Err("La pile ne contient pas assez de terme")
                }
            };
        }

        for t in tokens {
            match t {
                Token::Plus => {
                    let t1 = terme!();
                    let t2 =terme!();
                    self.stack.push(t1 + t2);
                },
                Token::Minus => {
                    let t1 = terme!();
                    let t2 =terme!();
                    self.stack.push(t1 - t2);
                },
                Token::Time => {
                    let t1 = terme!();
                    let t2 =terme!();
                    self.stack.push(t1 * t2);
                },
                Token::Div => {
                    let t1 = terme!();
                    let t2 =terme!();

                    if t2 == 0 {
                        return Err("Division par 0")
                    }

                    self.stack.push(t1 / t2)
                },
                Token::Number(value) => {
                    self.stack.push(value)
                }
            }
        }

        Ok(())
    }
}

enum Token {
    Plus,
    Minus,
    Time,
    Div,
    Number(i32),
}

enum ParsingState {
    Begin,
    Plus,
    Minus,
    Time,
    Div,
    Number
}

// Transforme des chaine de la forme "2 2 +" en tableau de toked de la forme [Number(2), Number(2), Plus]
fn tokenize(input: &String) -> Result<Vec<Token>, &'static str> {
    let mut state = ParsingState::Begin;
    let mut tokens : Vec<Token>  = Vec::new();
    let mut buffer = String::new();

    for c in input.chars() {
        if c == ' ' || c == '\n' { // L'espace sépare les tokens
            let token = match state {
                ParsingState::Plus => Token::Plus,
                ParsingState::Minus => Token::Minus,
                ParsingState::Time => Token::Time,
                ParsingState::Div => Token::Div,
                ParsingState::Number => {
                    let value : i32 = buffer.trim().parse().unwrap(); //Si l'automate est bien fait, ca ne crashera pas ici
                    Token::Number(value)
                },
                _ => return Err("État incohérent")
            };

            tokens.push(token);

            buffer.clear();
            state = ParsingState::Begin;

            continue;
        }
        else {
            state = match state {
                ParsingState::Begin => match c {
                    '+' => ParsingState::Plus,
                    '-' => ParsingState::Minus,
                    '*' => ParsingState::Time,
                    '/' => ParsingState::Div,
                    '0'...'9' => ParsingState::Number,
                    _ => return Err("Charactère inattendue"),
                },
                ParsingState::Plus => return Err("Charactère innattendue après +"),
                ParsingState::Minus => match c {
                    '0'...'9' => ParsingState::Number,
                    _ => return Err("Charactère innattendue après -")
                },
                ParsingState::Time => return Err("Charactère innattendue après *"),
                ParsingState::Div => return Err("Charactère innattendue après /"),
                ParsingState::Number => match c {
                    '0'...'9' => ParsingState::Number,
                    _ => return Err("Format de nombre non valide")
                }
            };

            buffer.push(c);
        }
    }

    Ok(tokens)
}

fn main() {
    let mut ctx = Context::new();

    loop {        
        let stdin = io::stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).unwrap();
        let tokens = match tokenize(&buffer) {
            Ok(t) => t,
            Err(msg) => {
                println!("Erreur d'entrée : {}", msg);
                continue;
            }
        };

        match ctx.run(tokens) {
            Ok(_) => {},
            Err(msg) => println!("Erreur : {}", msg)
        }

        println!("");
        println!("Affichage de la pile :");

        for value in &ctx.stack {
            println!("{}", value);
        }
        println!("");
    }
}
