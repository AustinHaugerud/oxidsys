use language::operations::Operation;

pub struct MissionCamSetPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2011;

pub const IDENT: &str = "mission_cam_set_position";

impl Operation for MissionCamSetPositionOp {
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
