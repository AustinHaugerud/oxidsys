use language::operations::Operation;

pub struct ChangeScreenMissionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2048;

pub const IDENT: &str = "change_screen_mission";

impl Operation for ChangeScreenMissionOp {
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
