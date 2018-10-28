use language::operations::Operation;

pub struct PropInstanceIsAnimatingOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1862;

pub const IDENT: &str = "prop_instance_is_animating";

impl Operation for PropInstanceIsAnimatingOp {
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
