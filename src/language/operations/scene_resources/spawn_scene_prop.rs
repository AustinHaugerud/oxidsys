use language::operations::Operation;

pub struct SpawnScenePropOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1974;

pub const IDENT: &str = "spawn_scene_prop";

impl Operation for SpawnScenePropOp {
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
