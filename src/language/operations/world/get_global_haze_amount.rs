use language::operations::Operation;

pub struct GetGlobalHazeAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 92;

pub const IDENT: &str = "get_global_haze_amount";

impl Operation for GetGlobalHazeAmountOp {
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
