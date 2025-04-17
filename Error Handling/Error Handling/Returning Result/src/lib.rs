pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // The error message should be: "`name` was empty; it must be nonempty."
        Err(String::from("`name` was empty; it must be nonempty."))
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}
