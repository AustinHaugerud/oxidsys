/**
* Parse expression. Expressions are of the format
* <operand> <param1> <param2> ... <paramN>;
*/

pub struct Expression {
    op_identifier: String,
    params: Vec<String>,
}

impl Expression {
    pub fn identifier(&self) -> &str {
        &self.op_identifier
    }

    pub fn parameters(&self) -> &Vec<String> {
        &self.params
    }
}

named!(extract_identifier<&str,&str>, take_until!(" "));
named!(extract_params_section<&str, &str>, take_until!(";"));

fn parse_expression(expression: &str) -> Result<Expression, ()> {
    let (rest, ident) = extract_identifier(expression).map_err(|_| {})?;
    let (_, params_section) = extract_params_section(rest).map_err(|_| {})?;

    let parts: Vec<String> = params_section
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

    Ok(Expression {
        op_identifier: ident.trim().to_owned(),
        params: parts,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_expression() {
        let expr = "val_add counter 1;";

        let result = super::parse_expression(expr).unwrap();

        assert_eq!(result.identifier(), "val_add");

        assert_eq!(result.parameters()[0], "counter");
        assert_eq!(result.parameters()[1], "1");
    }
}
