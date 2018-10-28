use language::operations::Operation;

pub struct MissionCamAnimateToPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2012;

pub const IDENT: &str = "mission_cam_animate_to_position";

impl Operation for MissionCamAnimateToPositionOp {
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
