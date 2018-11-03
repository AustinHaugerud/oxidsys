
#[derive(Parser)]
#[grammar = "language/oxid.pest"]
pub struct OxidParser;

fn filter(input : &str) -> String {
    input.replace("\n", "").replace("\r", "").replace("\t", "")
}

#[cfg(test)]
mod tests {

    use super::*;
    use pest::Parser;

    #[test]
    pub fn test1() {
        let content = r#"
            param p1;
            param p2;
            param p3;
            register reg20;

            local b = (10 + 2) * 2;
            b += 2 * 7;

            global c = (10 + b)^2 + (100 * (7-3) / 3);

            register r10;
            register r11;
            register r40;

            pregister p10;
            pregister p11;

            sregister s3;
            sregister s12;
        "#;

        let filtered = &filter(&content);

        let result = OxidParser::parse(Rule::file, filtered).unwrap();

        for pair in result {
            println!("{:?}", pair);
        }
    }

}
