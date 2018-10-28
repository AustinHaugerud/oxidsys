use language::operations::Operation;

pub struct MissionCamAnimateToScreenColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2009;

pub const IDENT: &str = "mission_cam_animate_to_screen_color";

impl Operation for MissionCamAnimateToScreenColorOp {
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
