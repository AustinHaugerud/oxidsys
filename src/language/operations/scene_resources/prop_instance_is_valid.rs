use language::operations::Operation;

pub struct PropInstanceIsValidOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1838;

pub const IDENT: &str = "prop_instance_is_valid";

impl Operation for PropInstanceIsValidOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
