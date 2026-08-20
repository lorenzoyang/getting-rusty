use crate::employee::Employee;

pub enum Command {
    Add {
        employee: Employee,
        department: String,
    },
    ListDepartment(String),
    ListAll,
    Help,
    Quit,
    Unknown(String),
}

// split a line of input into several words, then determine which type of command it is.
pub fn parse(input: &str) -> Command {
    let mut tokens: Vec<&str> = Vec::new();
    for token in input.split_whitespace() {
        tokens.push(token);
    }

    if tokens.is_empty() {
        return Command::Unknown(String::from(
            "Please enter a command. Type Help to view the usage instructions.",
        ));
    }

    let verb = tokens[0].to_lowercase();

    match verb.as_str() {
        "add" => match tokens.len() {
            // Add <name> to <department>
            4 if tokens[2].eq_ignore_ascii_case("to") => Command::Add {
                employee: Employee::new(String::from(tokens[1]), String::from("Member")),
                department: String::from(tokens[3]),
            },

            // Add <name> to <department> as <role>
            6 if tokens[2].eq_ignore_ascii_case("to") && tokens[4].eq_ignore_ascii_case("as") => {
                Command::Add {
                    employee: Employee::new(String::from(tokens[1]), String::from(tokens[5])),
                    department: String::from(tokens[3]),
                }
            }

            _ => Command::Unknown(String::from(
                "The format for Add should be: Add <name> to <department> or Add <name> to <department> as <role>",
            )),
        },

        "list" => match tokens.len() {
            1 => Command::ListAll,
            2 => Command::ListDepartment(String::from(tokens[1])),
            _ => Command::Unknown(String::from("List can be followed by at most one department name.")),
        },

        "help" => Command::Help,
        "quit" | "exit" => Command::Quit,
        _ => Command::Unknown(format!("Unrecognized command: {input}")),
    }
}
