
named!(extract_script_name<&str, &str>, take_until!("("));
named!(extract_script_params_section<&str, &str>, take_until!("{"));
named!(extract_script_block<&str, &str>, take_until!("}"));

fn parse_script_sections(input: &str) -> Result<(&str, &str, &str), ()> {
    let (r1, _) = extract_script_name(input).map_err(|_| {})?;
    let (r2, _) = extract_script_params_section(r1).map_err(|_| {})?;
    let (r3, _) = extract_script_block(r2).map_err(|_| {})?;

    Ok((r1.trim(), &r2[1..r2.len() - 1], &r3[1..]))
}

#[cfg(test)]
mod tests {

    use super::extract_script_name;

    #[test]
    fn test_parse_script_name_success() {
        let test1 = "foo()";
        let test2 = "foo ()";
        let test3 = "foo\t\n()";

        let res1 = extract_script_name(test1).unwrap();
        let res2 = extract_script_name(test2).unwrap();
        let res3 = extract_script_name(test3).unwrap();

        assert_eq!(res1.1, "foo");
        assert!(res1.0.starts_with("("));

        assert_eq!(res2.1, "foo ");
        assert!(res2.0.starts_with("("));

        assert_eq!(res3.1, "foo\t\n");
        assert!(res3.0.starts_with("("));
    }
}
