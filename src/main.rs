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

    // Mock Data
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
        employment_date: NaiveDate::from_ymd_opt(2022, 2, 27).unwrap(),
    });
    employee_list.push(Employee {
        name: "Darn".to_string(),
        department: Department::Back,
        salary: 1100,
        employment_date: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
    });
    employee_list.push(Employee {
        name: "Cus".to_string(),
        department: Department::Back,
        salary: 2500,
        employment_date: NaiveDate::from_ymd_opt(2020, 3, 9).unwrap(),
    });
    employee_list.push(Employee {
        name: "Bonu".to_string(),
        department: Department::Front,
        salary: 3000,
        employment_date: NaiveDate::from_ymd_opt(2021, 4, 21).unwrap(),
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
                "c" => add_employee(&mut employee_list),
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

fn add_employee(employee_list: &mut Vec<Employee>) {
    loop {
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
    let mut input: String = String::new();
    let mut sort_type: String = "Name".to_string();

    let enter_text = "\nEnter
>'x' to go back,
>'z' to sort by name or 'Z' for reverse order,
>'d' to sort by department or 'D' for reverse order,
>'c' to sort by salary or 'C' for reverse order,
>'v' to sort by employment date or 'V' for reverse order";
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
        t.sort();
    }
    loop {
        input.clear();

        println!("\n\n\n\n");
        println!("Sorted by {}", sort_type);
        println!("------------------");
        match department {
            "All Departments" => {
                for (name, department, salary, empolyment_date) in &t {
                    println!(
                        "> {} | {} | {} Eur | {}",
                        name, department, salary, empolyment_date
                    );
                }
            }
            "Front" => {
                for (name, department, salary, empolyment_date) in &t {
                    if department == "Front" {
                        println!(
                            "> {} | {} | {} Eur | {}",
                            name, department, salary, empolyment_date
                        );
                    }
                }
            }
            "Back" => {
                for (name, department, salary, empolyment_date) in &t {
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
        println!("{}", &enter_text);
        match stdin().read_line(&mut input) {
            Ok(_) => match input.as_str().trim() {
                "x" => {
                    break;
                }
                "z" => {
                    sort_type = "Name".to_string();
                    t.sort();
                    continue;
                }
                "Z" => {
                    sort_type = "Name".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            b.0.cmp(&a.0)
                        },
                    );
                    continue;
                }
                "c" => {
                    sort_type = "Salary".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            b.2.cmp(&a.2)
                        },
                    );
                    continue;
                }
                "C" => {
                    sort_type = "Salary".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            a.2.cmp(&b.2)
                        },
                    );
                    continue;
                }
                "v" => {
                    sort_type = "Empployment Date".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            a.3.cmp(&b.3)
                        },
                    );
                    continue;
                }
                "V" => {
                    sort_type = "Empployment Date".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            b.3.cmp(&a.3)
                        },
                    );
                    continue;
                }
                "d" => {
                    sort_type = "Department".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            b.1.cmp(&a.1)
                        },
                    );
                    continue;
                }
                "D" => {
                    sort_type = "Department".to_string();
                    t.sort_by(
                        |a: &(String, String, String, String),
                         b: &(String, String, String, String)| {
                            a.1.cmp(&b.1)
                        },
                    );
                    continue;
                }
                _ => {
                    println!(
                        "Please enter a letter from the list below.
                    {}",
                        enter_text
                    )
                }
            },
            Err(_) => {
                println!("Something went wrong reading the input");
                continue;
            }
        }
    }
}
