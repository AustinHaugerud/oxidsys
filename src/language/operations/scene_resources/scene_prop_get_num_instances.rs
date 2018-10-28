use language::operations::Operation;

pub struct ScenePropGetNumInstancesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1810;

pub const IDENT: &str = "scene_prop_get_num_instances";

impl Operation for ScenePropGetNumInstancesOp {
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
