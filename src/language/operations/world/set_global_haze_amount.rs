use language::operations::Operation;

pub struct SetGlobalHazeAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 93;

pub const IDENT: &str = "set_global_haze_amount";

impl Operation for SetGlobalHazeAmountOp {
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
