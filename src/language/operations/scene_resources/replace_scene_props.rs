use language::operations::Operation;

pub struct ReplaceScenePropsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1890;

pub const IDENT: &str = "replace_scene_props";

impl Operation for ReplaceScenePropsOp {
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
