use language::operations::Operation;

pub struct SceneSpawnedItemGetInstanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1833;

pub const IDENT: &str = "scene_spawned_item_get_instance";

impl Operation for SceneSpawnedItemGetInstanceOp {
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
