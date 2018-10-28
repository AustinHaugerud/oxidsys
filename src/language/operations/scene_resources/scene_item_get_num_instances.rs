use language::operations::Operation;

pub struct SceneItemGetNumInstancesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1830;

pub const IDENT: &str = "scene_item_get_num_instances";

impl Operation for SceneItemGetNumInstancesOp {
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
