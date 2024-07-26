use chrono::NaiveDate;
use std::fmt;
use std::io::stdin;
enum Department {
    Front,
    Back,
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Department::Front => write!(f, "Front"),
            Department::Back => write!(f, "Back"),
        }
    }
}

struct Employee {
    name: String,
    department: Department,
    salary: u16,
    employment_date: NaiveDate,
}

fn main() {
    let mut pressed_x = false;
    let mut employee_list: Vec<Employee> = Vec::new();
    let mut input = String::new();

    employee_list.push(Employee {
        name: "Lucas".to_string(),
        department: Department::Back,
        salary: 2000,
        employment_date: NaiveDate::from_ymd_opt(2022, 10, 5).unwrap(),
    });
    employee_list.push(Employee {
        name: "Dan".to_string(),
        department: Department::Front,
        salary: 1200,
        employment_date: NaiveDate::from_ymd_opt(2021, 6, 18).unwrap(),
    });
    employee_list.push(Employee {
        name: "Mark".to_string(),
        department: Department::Front,
        salary: 1800,
        employment_date: NaiveDate::from_ymd_opt(2022, 1, 31).unwrap(),
    });

    println!("Employees count before {}", employee_list.len());

    //add_employee(&mut map, &mut pressed_x);
    while !pressed_x {
        let enter_text = "\nEnter \n>'x' to exit, \n>'z' to show department employees 
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
                    list_of_employees(&employee_list, "All Departments");
                }
                "z" => loop {
                    println!("Enter Department\n> Front\n> Back\n");
                    input.clear();

                    match stdin().read_line(&mut input) {
                        Ok(_) => match input.as_str().trim() {
                            "Front" => list_of_employees(&employee_list, "Front"),
                            "Back" => list_of_employees(&employee_list, "Back"),
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
                "c" => add_employee(&mut employee_list, &mut pressed_x),
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
    println!("Map length after {}", employee_list.len());
}

fn add_employee(employee_list: &mut Vec<Employee>, pressed_x: &mut bool) {
    while !*pressed_x {
        let mut input: String = String::new();
        let mut name: String = String::new();
        let department: Department;
        let salary: u16;
        let date: NaiveDate;
        input.clear();

        println!("Type Name: Sam");
        match stdin().read_line(&mut input) {
            Ok(_) => {
                name = input.trim().to_string().clone();
                println!("Employee Name {}", &name,);
            }
            Err(_) => {
                println!("Invalid Name");
                continue;
            }
        };
        loop {
            input.clear();
            println!("Enter a Department (Back/Front): Front");
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.as_str().trim() {
                        "Front" => department = Department::Front,

                        "Back" => department = Department::Back,
                        _ => {
                            println!("Invalid Department: {}", &input);
                            continue;
                        }
                    }
                    break;
                }
                Err(_) => {
                    println!("Invalid Department: {}", &input);
                    continue;
                }
            };
        }
        loop {
            input.clear();
            println!("Enter a Salary: 2000");
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    if let Ok(number) = input.trim().parse::<u16>() {
                        salary = number;

                        break;
                    } else {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
        loop {
            input.clear();
            println!("Enter Employment Date: 2024.01.31");
            match stdin().read_line(&mut input) {
                Ok(_) => {
                    let vect: Vec<&str> = input.trim().split('.').collect();
                    if let Some(valid) = NaiveDate::from_ymd_opt(
                        vect[0].parse::<i32>().unwrap(),
                        vect[1].parse::<u32>().unwrap(),
                        vect[2].parse::<u32>().unwrap(),
                    ) {
                        date = valid;
                        break;
                    } else {
                        println!("Invalid date! {}", &input);
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
        employee_list.push(Employee {
            name: name,
            department: department,
            salary: salary,
            employment_date: date,
        });
        println!(
            "\n\nEmpolyee added: {} {} {} {}",
            &employee_list.last().unwrap().name,
            &employee_list.last().unwrap().department,
            &employee_list.last().unwrap().salary,
            &employee_list.last().unwrap().employment_date,
        );
        break;
    }
}

fn list_of_employees(employee_list: &Vec<Employee>, department: &str) {
    let mut t: Vec<(String, String, String, String)> = Vec::new();

    for employee in employee_list {
        t.push((
            employee.name.clone(),
            match employee.department {
                Department::Front => "Front".to_string(),
                Department::Back => "Back".to_string(),
            },
            employee.salary.to_string(),
            employee.employment_date.to_string(),
        ));
    }
    t.sort();
    println!("");
    println!("------------------");
    match department {
        "All Departments" => {
            for (name, department, salary, empolyment_date) in t {
                println!(
                    "> {} | {} | {} Eur | {}",
                    name, department, salary, empolyment_date
                );
            }
        }
        "Front" => {
            for (name, department, salary, empolyment_date) in t {
                if department == "Front" {
                    println!(
                        "> {} | {} | {} Eur | {}",
                        name, department, salary, empolyment_date
                    );
                }
            }
        }
        "Back" => {
            for (name, department, salary, empolyment_date) in t {
                if department == "Back" {
                    println!(
                        "> {} | {} | {} Eur | {}",
                        name, department, salary, empolyment_date
                    );
                }
            }
        }
        _ => {}
    }
    println!("------------------");
    println!("");
}
