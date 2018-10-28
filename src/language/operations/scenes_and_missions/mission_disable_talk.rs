use language::operations::Operation;

pub struct MissionDisableTalkOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1936;

pub const IDENT: &str = "mission_disable_talk";

impl Operation for MissionDisableTalkOp {
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
