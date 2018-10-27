pub fn convert_to_identifier_with_no_lowercase(basis: &str) -> String {
    basis
        .replace(" ", "_")
        .replace("'", "_")
        .replace("`", "_")
        .replace("(", "_")
        .replace(")", "_")
        .replace("-", "_")
        .replace(",", "")
        .replace("|", "")
        .replace("\t", "_")
}

pub fn convert_to_identifier(basis: &str) -> String {
    convert_to_identifier_with_no_lowercase(basis).to_ascii_lowercase()
}

pub fn replace_spaces(basis: &str) -> String {
    basis.replace(" ", "_").replace("\t", "_")
}
