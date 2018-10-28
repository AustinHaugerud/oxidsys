const GLOBAL_PREFIX: &str = "$";
const LOCAL_PREFIX: &str = ":";

named!(get_scope_char<&str, &str>, alt!(tag!(GLOBAL_PREFIX) | tag!(LOCAL_PREFIX)));

#[cfg(test)]
mod tests {
    use super::get_scope_char;
    use super::GLOBAL_PREFIX;
    use super::LOCAL_PREFIX;

    #[test]
    fn test_get_scope_char_gets_global() {
        let source = GLOBAL_PREFIX.to_owned() + "g_harvesting_season";
        let res = get_scope_char(&source);

        let (_, scope_char) = res.unwrap();

        assert_eq!(scope_char, GLOBAL_PREFIX);
    }

    #[test]
    fn test_get_scope_char_gets_local() {
        let source = LOCAL_PREFIX.to_owned() + "g_harvesting_season";
        let res = get_scope_char(&source);
        let (_, scope_char) = res.unwrap();
        assert_eq!(scope_char, LOCAL_PREFIX);
    }

    #[test]
    #[should_panic]
    fn test_get_scope_char_fails_missing() {
        let source = "g_harvesting_season";
        get_scope_char(&source).unwrap();
    }
}
