use language::operations::Operation;

pub struct GetSceneBoundariesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1799;

pub const IDENT: &str = "get_scene_boundaries";

impl Operation for GetSceneBoundariesOp {
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
