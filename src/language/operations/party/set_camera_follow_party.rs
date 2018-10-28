use language::operations::Operation;

pub struct SetCameraFollowPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1021;

pub const IDENT: &str = "set_camera_follow_party";

impl Operation for SetCameraFollowPartyOp {
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
