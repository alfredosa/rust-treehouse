use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    age: i8,
    action: VisitorAction
}

#[derive(Debug)]
enum VisitorAction {
  Accept,
  AcceptWithNote {note: String},
  Refuse,
  Probation
}

impl Visitor {
    fn new(name: &str, age: i8, action: VisitorAction) -> Self {
        Self {
            name: name.to_lowercase(),
            age,
            action
        }
    }
    fn greet_visitor(&self) {
        println!("Hello dear fellow list member");
    }
}
fn whats_your_age()->i8 {
  let mut your_age = String::new();
  stdin()
        .read_line(&mut your_age)
        .expect("Failed to read Line");
  let age = your_age.trim().parse().expect("Input not an integer");
  return age

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
            Visitor::new("bert", 45, VisitorAction::Accept),
            Visitor::new("steve", 15,VisitorAction::AcceptWithNote { note: String::from("Milk is in the fridge bruh") }),
            Visitor::new("alfredo", 29, VisitorAction::Refuse),
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
                        println!("The final list of visitors in the Threehouse:");
                        println!("{:#?}",visitor_list);
                        break;
                    }
                    else {
                        println!("{} is not in the list, What's your age?",your_name);
                        let age = whats_your_age();
                        let action_for_new_comer = if age >= 14 { VisitorAction::Probation } else { VisitorAction::Refuse };
                        visitor_list.push(Visitor::new(&your_name, age,  action_for_new_comer));
                }
            }

            }
        }
    }
