mod command;
mod employee;

use command::{Command, parse};
use employee::Employee;

use std::collections::HashMap;
use std::io::{self, Write};

fn has_employee(members: &Vec<Employee>, name: &str) -> bool {
    for employee in members {
        if employee.name() == name {
            return true;
        }
    }
    false
}

fn add_employee(
    company: &mut HashMap<String, Vec<Employee>>,
    employee: Employee,
    department: String,
) {
    let members = company.entry(department.clone()).or_insert(Vec::new());

    if has_employee(members, employee.name()) {
        println!("{} is already in {department} 了。", employee.name());
        return;
    }

    println!("Added {} to {department}。", employee.display_line());
    members.push(employee);
}

// sorted alphabetically by name.
fn print_department(company: &HashMap<String, Vec<Employee>>, department: &str) {
    match company.get(department) {
        Some(members) => {
            let mut sorted = members.clone();
            sorted.sort();

            println!("{department}（{} people）：", sorted.len());
            for employee in &sorted {
                println!("  - {}", employee.display_line());
            }
        }
        None => println!("No department named '{department}'."),
    }
}

/// grouped by department, with both department names and employee names sorted alphabetically.
fn list_all(company: &HashMap<String, Vec<Employee>>) {
    if company.is_empty() {
        println!("No employees recorded yet.");
        return;
    }

    // Hash map traversal order is random, so collect department names and sort them for consistent output
    let mut departments: Vec<String> = Vec::new();
    for department in company.keys() {
        departments.push(department.clone());
    }
    departments.sort();

    for department in &departments {
        print_department(company, department);
    }
}

fn print_help() {
    println!("Available commands:");
    println!(
        "  Add <name> to <department>                 Add an employee (default role: \"Member\")"
    );
    println!("  Add <name> to <department> as <role>  Add an employee with a specified role");
    println!("  List <department>                     List everyone in a department");
    println!("  List                                  List everyone in the company by department");
    println!("  Help                                  Show this help message");
    println!("  Quit                                  Exit the program");
}

fn main() {
    let mut company: HashMap<String, Vec<Employee>> = HashMap::new();

    println!("Employee Directory. Type Help to view commands, Quit to exit.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output");

        let mut line = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read input");

        if bytes_read == 0 {
            break;
        }

        let line = line.trim();

        match parse(line) {
            Command::Add {
                employee,
                department,
            } => add_employee(&mut company, employee, department),
            Command::ListDepartment(department) => print_department(&company, &department),
            Command::ListAll => list_all(&company),
            Command::Help => print_help(),
            Command::Quit => break,
            Command::Unknown(message) => println!("{message}"),
        }
    }

    println!("Exiting Employee Directory. Goodbye!");
}
