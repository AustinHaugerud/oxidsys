use language::operations::Operation;

pub struct MissionCamSetModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2001;

pub const IDENT: &str = "mission_cam_set_mode";

impl Operation for MissionCamSetModeOp {
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
