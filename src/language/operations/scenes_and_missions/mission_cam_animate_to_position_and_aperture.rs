use language::operations::Operation;

pub struct MissionCamAnimateToPositionAndApertureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2016;

pub const IDENT: &str = "mission_cam_animate_to_position_and_aperture";

impl Operation for MissionCamAnimateToPositionAndApertureOp {
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
