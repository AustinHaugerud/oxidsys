use language::operations::Operation;

pub struct SceneItemGetInstanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1831;

pub const IDENT: &str = "scene_item_get_instance";

impl Operation for SceneItemGetInstanceOp {
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
