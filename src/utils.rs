fn validate_name(name: &str) -> bool {
    let error_mensage: str;
    for ch in name.chars() {
        if !ch.is_alphabetic() || !ch.is_space {
            error_mensage.append("Name can only have letters");
            return false;
        }
    }
    true
}
