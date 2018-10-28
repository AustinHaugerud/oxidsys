use language::operations::Operation;

pub struct TroopIsWoundedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1508;

pub const IDENT: &str = "troop_is_wounded";

impl Operation for TroopIsWoundedOp {
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
