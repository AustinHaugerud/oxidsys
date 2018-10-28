use language::operations::Operation;

pub struct AutoSetMetaMissionAtEndCommitedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1305;

pub const IDENT: &str = "auto_set_meta_mission_at_end_commited";

impl Operation for AutoSetMetaMissionAtEndCommitedOp {
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
