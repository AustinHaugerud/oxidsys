use language::operations::Operation;

pub struct ScenePropEnableAfterTimeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1800;

pub const IDENT: &str = "scene_prop_enable_after_time";

impl Operation for ScenePropEnableAfterTimeOp {
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
