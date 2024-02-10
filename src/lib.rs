use clap::ArgMatches;
use colored::Colorize;
use std::collections::HashMap;
use std::io::Write;
use std::{fmt, fs};
use std::fs::File;

pub fn run(matches: ArgMatches) {
    let items = matches
        .get_many::<String>("values")
        .unwrap_or_default()
        .map(|v| v.clone())
        .collect::<Vec<_>>();

    if matches.get_flag("create") {
        if let Err(e) = create() {
            eprintln!("Error: {}", e);
        }
    } else if matches.get_flag("list") {
        if let Err(e) = list() {
            eprintln!("Error: {}", e);
        }
    } else if matches.get_flag("add") {
        if let Err(e) = add_items(items) {
            eprintln!("Error: {}", e);
        }
    }
    else if matches.get_flag("finish") {
        if let Err(e) = finish_items(items) {
            eprintln!("Error: {}", e);
        }
    }
}

fn list() -> Result<usize, Error> {
    let contents = match fs::read_to_string("todo_rs.todo") {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::FileInaccessible);
        }
    };

    let values = match parse(contents) {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };

    if values.len() == 0 {
        return Err(Error::NoContent);
    }

    for (value, is_done) in values {
        if is_done == "true" {
            print!("{:3} ", value.strikethrough())
        } else {
            print!("{:3} ", value.bold())
        }
    }
    println!();

    Ok(0)
}

fn create() -> Result<usize, Error> {
    if let Err(_) = fs::write("todo_rs.todo", "###TODO_RS###\n") {
        return Err(Error::FileInaccessible);
    }

    Ok(0)
}

fn add_items(items: Vec<String>) -> Result<usize, Error> {
    let mut file = match fs::OpenOptions::new().append(true).open("todo_rs.todo") {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::FileInaccessible);
        }
    };

    for item in items {
        if let Err(_) = file.write(format!("{} false\n", item).as_bytes()) {
            return Err(Error::WriteFailed);
        }
    }

    Ok(0)
}

fn finish_items(items: Vec<String>) -> Result<usize, Error> {
    let mut contents = match fs::read_to_string("todo_rs.todo") {
        Ok(t) => t,
        Err(_) => return Err(Error::FileInaccessible),
    };
    let mut file = match File::create("todo_rs.todo") {
        Ok(t) => t,
        Err(_) => { return Err(Error::FileInaccessible); }
    };

    let mut substring1: String;
    let mut substring2: String;
    for item in items {
        substring1 = format!("{} false", item);

        substring2 = format!("{} true", item);

        contents = contents.replace(&substring1, substring2.as_str());

        if let Err(e) = file.write(contents.as_bytes()) {
            println!("{e}");
            return Err(Error::WriteFailed);
        }

    }

    Ok(0)
}

fn parse(contents: String) -> Result<HashMap<String, String>, Error> {
    let lines = contents.lines();
    let mut output: HashMap<String, String> = HashMap::new();

    let mut tmp_buff: Vec<String>;
    for line in lines {
        if line == "###TODO_RS###" {
            continue;
        }

        tmp_buff = line.split(" ").map(|line| line.to_string()).collect();

        if tmp_buff.len() != 2 && tmp_buff.len() != 0 {
            return Err(Error::ParseError);
        }
        if tmp_buff[1] != "true" && tmp_buff[1] != "false" {
            return Err(Error::ParseError);
        }

        output.insert(tmp_buff[0].clone(), tmp_buff[1].clone());
    }

    Ok(output)
}

#[derive(Debug, PartialEq)]
enum Error {
    ParseError,
    FileInaccessible,
    NoContent,
    WriteFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &crate::Error::ParseError {
            return write!(f, "Cannot parse todo_rs.todo");
        } else if self == &crate::Error::FileInaccessible {
            return write!(f, "Cannot read todo_rs.todo. Perhaps it is not created?");
        } else if self == &crate::Error::NoContent {
            return write!(
                f,
                "No content in todo_rs.todo. Perhaps you have not added any items yet?"
            );
        } else if self == &crate::Error::WriteFailed {
            return write!(f, "Unable to write to todo_rs.todo.");
        }

        write!(f, "idk")
    }
}

impl std::error::Error for Error {}
