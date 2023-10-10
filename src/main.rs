use std::collections::HashMap;
use std::io::stdin;

enum Department {
    Front(String),
    Back(String),
}

fn add_employee() -> Option<(String, Department)> {
    let mut pressed_x = false;
    while !pressed_x {
        println!("Enter a command: Add Sam to Back");
        let mut input = String::new();
        let mut action = String::new();
        let mut name = String::new();
        let mut department: Department;
        match stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Your input was {}", &input);
                let vect: Vec<&str> = input.split_whitespace().collect();
                action = vect[0].to_string();
                name = vect[1].to_string();
                match vect[3] {
                    "Front" => department = Department::Front("Front".to_string()),
                    "Back" => department = Department::Back("Back".to_string()),
                    _ => {
                        println!("Not a department: {}", vect[3]);
                        continue;
                    }
                }
                pressed_x = true;
                return Some((name, department));
            }
            Err(_) => {
                println!("Something went wrong reading the input");

                continue;
            }
        };
    }
    None
}

fn main() {
    let mut map: HashMap<String, Department> = HashMap::new();
    map.insert("Lucas".to_string(), Department::Front("Front".to_string()));
    map.insert("Dan".to_string(), Department::Front("Back".to_string()));
    map.insert("Mark".to_string(), Department::Front("Front".to_string()));

    map.insert()add_employee();
    println!("{}", map.len());
}
