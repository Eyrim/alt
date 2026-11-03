pub const IFS: &'static str = "IFS";
pub const NEWLINE: &'static str = "\n";

pub fn var_or<T>(var: &str, fallback: T) -> String
where
    T: Into<String> {
        match std::env::var(var) {
            Ok(value) => value,
            //TODO: logging
            Err(_) => fallback.into()
        }
}

