use language::operations::Operation;

pub struct MissionCamSetTargetAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2017;

pub const IDENT: &str = "mission_cam_set_target_agent";

impl Operation for MissionCamSetTargetAgentOp {
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
