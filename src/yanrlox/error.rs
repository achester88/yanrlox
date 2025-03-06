pub struct Error {
    line: usize,
    column: usize,
    message: String,
    show_pos: bool,
    show_line: bool,
    description: Option<String>
}

impl Error {
    pub fn simple_error(message: &str) -> Self {
        Error {line: 0, column: 0, message: message.to_owned(), show_pos: false, show_line: false, description: None}
    }

    pub fn line_error(message: &str, line: usize, description: &str) -> Self {
        Error {line: line, column: 0, message: message.to_owned(), show_pos: false, show_line: true, description: Some(description.to_owned())}
    }

    pub fn pos_error(message: &str, line: usize, column: usize, description: &str) -> Self {
        Error {line: line, column: column, message: message.to_owned(), show_pos: true, show_line: true, description: Some(description.to_owned())}
    }
}

pub fn throw_error(source: String, error: Error) {
    if !error.show_pos && !error.show_line {
        println!("\x1b[91mError\x1b[0m: {}", error.message);
    } else {
        let by_line: Vec<&str> = source.split("\n").collect();
        if !error.show_pos {
            println!("\x1b[91mError\x1b[0m: {}\n\n[{}] {}", error.message, error.line, by_line[error.line-1]);
        } else {
            let offset: usize = (2 + (error.line.checked_ilog10().unwrap_or(0) + 1) + (error.column as u32)).try_into().unwrap();
            println!("\x1b[91mError\x1b[0m: {}\n\n[{}] {}\n{}\x1b[33m^\x1b[0m", error.message, error.line, by_line[error.line-1], " ".repeat(offset));
        }
    }
        
    if error.description.is_some() {
        println!("{}", error.description.unwrap());
    }

}
