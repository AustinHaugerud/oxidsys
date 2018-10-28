use language::operations::Operation;

pub struct ScenePropSetVisibilityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1813;

pub const IDENT: &str = "scene_prop_set_visibility";

impl Operation for ScenePropSetVisibilityOp {
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
