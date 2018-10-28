use language::operations::Operation;

pub struct SceneSetDayTimeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1266;

pub const IDENT: &str = "scene_set_day_time";

impl Operation for SceneSetDayTimeOp {
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
