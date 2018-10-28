use language::operations::Operation;

pub struct MissionCamSetApertureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2014;

pub const IDENT: &str = "mission_cam_set_aperture";

impl Operation for MissionCamSetApertureOp {
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
