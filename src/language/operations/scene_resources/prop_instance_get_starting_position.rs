use language::operations::Operation;

pub struct PropInstanceGetStartingPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1851;

pub const IDENT: &str = "prop_instance_get_starting_position";

impl Operation for PropInstanceGetStartingPositionOp {
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
