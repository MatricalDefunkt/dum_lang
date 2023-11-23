use regex::Regex;

pub fn get_string(input_line: String) -> String {
    let mut string_constructor = String::new();
    let mut is_string = false;
    let mut is_escape = false;
    for char in input_line.chars() {
        if is_escape {
            string_constructor.push(char);
            is_escape = false;
        } else if char == '\\' {
            is_escape = true;
        } else if char == '\'' {
            is_string = !is_string;
        }
        if is_string && !is_escape {
            string_constructor.push(char);
        }
    }
    string_constructor.insert(0, '\'');
    string_constructor.push('\'');
    string_constructor
}

pub fn is_var(input_string: String) -> bool {
    Regex::new(r"^[a-z]+$").unwrap().is_match(&input_string)
}

pub fn is_int(input_string: String) -> bool {
    Regex::new(r"^[0-9]+$").unwrap().is_match(&input_string)
}

pub fn is_float(input_string: String) -> bool {
    Regex::new(r"^[0-9]+\.[0-9]+$")
        .unwrap()
        .is_match(&input_string)
}

pub fn is_string(input_string: String) -> bool {
    Regex::new(r"^'.*'$").unwrap().is_match(&input_string)
}
