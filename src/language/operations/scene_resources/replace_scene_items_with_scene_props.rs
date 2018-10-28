use language::operations::Operation;

pub struct ReplaceSceneItemsWithScenePropsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1891;

pub const IDENT: &str = "replace_scene_items_with_scene_props";

impl Operation for ReplaceSceneItemsWithScenePropsOp {
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
