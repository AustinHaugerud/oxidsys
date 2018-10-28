use language::operations::Operation;

pub struct ScenePropGetInstanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1811;

pub const IDENT: &str = "scene_prop_get_instance";

impl Operation for ScenePropGetInstanceOp {
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
