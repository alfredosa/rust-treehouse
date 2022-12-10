use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read Line");
    return your_name.trim().to_lowercase();
}

fn main() {
        let mut visitor_list = vec![
            Visitor::new("bert", "Hello Bert, welcome to the house"),
            Visitor::new("steve", "Hello Steve, welcome to the house"),
            Visitor::new("alfredo", "Hello Alfredo, welcome to the house"),
        ];
        loop {
            println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
            let your_name = what_is_your_name();
            let known_visitor = visitor_list
                .iter()
                .find(|visitor| visitor.name == your_name);
            match known_visitor {
                Some(visitor)=> visitor.greet_visitor(),
                None => 
                {
                    if your_name.is_empty() {
                        break;
                    }  
                    else {
                        println!("{} is not in the list",your_name);
                        visitor_list.push(Visitor::new(&your_name,&format!("hey hey {}!",your_name)));
                }
            }

            }
        }
    } 