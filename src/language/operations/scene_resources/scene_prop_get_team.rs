use language::operations::Operation;

pub struct ScenePropGetTeamOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1817;

pub const IDENT: &str = "scene_prop_get_team";

impl Operation for ScenePropGetTeamOp {
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
