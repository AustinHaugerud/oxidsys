use language::operations::Operation;

pub struct MissionCamClearTargetAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2018;

pub const IDENT: &str = "mission_cam_clear_target_agent";

impl Operation for MissionCamClearTargetAgentOp {
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
