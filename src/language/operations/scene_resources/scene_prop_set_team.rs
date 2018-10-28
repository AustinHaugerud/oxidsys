use language::operations::Operation;

pub struct ScenePropSetTeamOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1818;

pub const IDENT: &str = "scene_prop_set_team";

impl Operation for ScenePropSetTeamOp {
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
