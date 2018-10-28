use language::operations::Operation;

pub struct MissionCamSetScreenColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2008;

pub const IDENT: &str = "mission_cam_set_screen_color";

impl Operation for MissionCamSetScreenColorOp {
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
