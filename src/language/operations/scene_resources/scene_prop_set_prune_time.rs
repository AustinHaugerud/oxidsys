use language::operations::Operation;

pub struct ScenePropSetPruneTimeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1819;

pub const IDENT: &str = "scene_prop_set_prune_time";

impl Operation for ScenePropSetPruneTimeOp {
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
