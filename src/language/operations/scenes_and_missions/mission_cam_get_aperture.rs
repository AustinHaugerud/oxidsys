use language::operations::Operation;

pub struct MissionCamGetApertureOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2013;

pub const IDENT: &str = "mission_cam_get_aperture";

impl Operation for MissionCamGetApertureOp {
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
