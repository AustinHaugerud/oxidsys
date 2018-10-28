use language::operations::Operation;

pub struct SetSpawnEffectorScenePropKindOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 426;

pub const IDENT: &str = "set_spawn_effector_scene_prop_kind";

impl Operation for SetSpawnEffectorScenePropKindOp {
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
