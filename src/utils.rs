use std::env;

pub fn check_var_set(var_name: &'static str) -> bool {
    match env::var(var_name) {
        Ok(value) => value.parse::<u8>().unwrap_or(0) == 1,
        Err(_) => false,
    }
}
