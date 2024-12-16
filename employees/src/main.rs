use std::collections::HashMap;
use std::io;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Department {
    Sales,
    Engineering,
}

fn main() {
    let mut employees: HashMap<Department, Vec<String>> = HashMap::new();
    loop {
        println!("Enter 0 for Sales, 1 for Engineering, 2 to quit:");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Failed to read line");
        let department: Department = match department.trim().parse() {
            Ok(0) => Department::Sales,
            Ok(1) => Department::Engineering,
            Ok(2) => {
                break;
            }
            _ => {
                continue;
            }
        };
        println!("Enter employee name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();
        println!("Adding {} to {:?}", name, department);
        let existing = employees.entry(department).or_insert(Vec::new());
        existing.push(name);
    }

    for (department, mut names) in employees {
        print_employees_of_department(&mut names, &department);
    }
}

// sorts the names in place, then prints them
fn print_employees_of_department(
    names: &mut Vec<String>,
    department: &Department
) {
    println!("{:?} department:", department);
    names.sort();
    for name in names {
        println!("{}", name);
    }
}
