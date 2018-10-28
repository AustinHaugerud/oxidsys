use language::operations::Operation;

pub struct MissionCamAnimateToScreenColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2009;

pub const IDENT: &str = "mission_cam_animate_to_screen_color";

impl Operation for MissionCamAnimateToScreenColorOp {
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
