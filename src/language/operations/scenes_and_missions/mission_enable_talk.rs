use language::operations::Operation;

pub struct MissionEnableTalkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1935;

pub const IDENT: &str = "mission_enable_talk";

impl Operation for MissionEnableTalkOp {
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
