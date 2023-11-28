use regex::Regex;

pub fn is_var(input_string: &String) -> bool {
    Regex::new(r"^([a-zA-Z]+[0-9]*)$")
        .unwrap()
        .is_match(&input_string)
}

pub fn is_int(input_string: &String) -> bool {
    Regex::new(r"^[0-9]+$").unwrap().is_match(&input_string)
}

pub fn is_float(input_string: &String) -> bool {
    Regex::new(r"^[0-9]+\.[0-9]+$")
        .unwrap()
        .is_match(&input_string)
}

pub fn is_string(input_string: &String) -> bool {
    Regex::new(r"^'.*'$").unwrap().is_match(&input_string)
}
