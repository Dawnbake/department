use std::collections::HashMap;
use std::io::stdin;

enum Department {
    Front(String),
    Back(String),
}

fn add_employee(map: &mut HashMap<String, Department>, pressed_x: &mut bool) {
    while !*pressed_x {
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

                println!(
                    "Successfully added {} to department {}",
                    &name,
                    match department {
                        Department::Front(ref department) => department,
                        Department::Back(ref department) => department,
                    }
                );
                map.insert(name, department);
                action.clear();
                input.clear();
                break;
            }
            Err(_) => {
                println!("Something went wrong reading the input");
                continue;
            }
        };
    }
}

fn list_of_employees(map: &mut HashMap<String, Department>, department: &str) {
    match department {
        "All Departments" => {}
        "Front" => {}
        "Back" => {}
        _ => {}
    }
}

fn main() {
    let mut pressed_x = false;
    let mut map: HashMap<String, Department> = HashMap::new();
    let mut input = String::new();

    map.insert("Lucas".to_string(), Department::Front("Front".to_string()));
    map.insert("Dan".to_string(), Department::Front("Back".to_string()));
    map.insert("Mark".to_string(), Department::Front("Front".to_string()));
    println!("Map length before {}", map.len());

    add_employee(&mut map, &mut pressed_x);
    while !pressed_x {
        let enter_text = "\nEnter \n>'x' to exit, \n>'z' to show department employees and 
>'c' to add another employee \n>'Z' to show all employees";
        println!("{}", &enter_text);
        input.clear();
        match stdin().read_line(&mut input) {
            Ok(_) => match input.as_str().trim() {
                "x" => {
                    pressed_x = true;
                    break;
                }
                "Z" => {
                    list_of_employees(&mut map, "All Departments");
                }
                "z" => loop {
                    println!("Enter Department\n> Front\n> Back\n");
                    input.clear();

                    match stdin().read_line(&mut input) {
                        Ok(_) => match input.as_str().trim() {
                            "Front" => list_of_employees(&mut map, "Front"),
                            "Back" => list_of_employees(&mut map, "Back"),
                            _ => {
                                println!("No department named {}", input);
                                continue;
                            }
                        },
                        Err(_) => {
                            println!("Something went wrong reading the input");
                            continue;
                        }
                    }
                    break;
                },
                "c" => add_employee(&mut map, &mut pressed_x),
                _ => println!("{}", &enter_text),
            },
            Err(_) => {
                println!("Something went wrong reading the input");
                continue;
            }
        }
        if pressed_x {
            break;
        }
    }
    println!("Map length after {}", map.len());
}
