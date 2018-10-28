use language::operations::Operation;

pub struct ScenePropSetCurHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1820;

pub const IDENT: &str = "scene_prop_set_cur_hit_points";

impl Operation for ScenePropSetCurHitPointsOp {
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
