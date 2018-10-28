use language::operations::Operation;

pub struct StopAllSoundsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 609;

pub const IDENT: &str = "stop_all_sounds";

impl Operation for StopAllSoundsOp {
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
