use language::operations::Operation;

pub struct SetSpawnEffectorScenePropIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 427;

pub const IDENT: &str = "set_spawn_effector_scene_prop_id";

impl Operation for SetSpawnEffectorScenePropIdOp {
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
