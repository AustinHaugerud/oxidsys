use language::operations::Operation;

pub struct MissionTplEntrySetOverrideFlagsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1940;

pub const IDENT: &str = "mission_tpl_entry_set_override_flags";

impl Operation for MissionTplEntrySetOverrideFlagsOp {
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
