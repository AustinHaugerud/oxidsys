use language::operations::Operation;

pub struct PropInstanceReceiveDamageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1877;

pub const IDENT: &str = "prop_instance_receive_damage";

impl Operation for PropInstanceReceiveDamageOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
