use language::operations::Operation;

pub struct MissionTplEntryClearOverrideItemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1941;

pub const IDENT: &str = "mission_tpl_entry_clear_override_items";

impl Operation for MissionTplEntryClearOverrideItemsOp {
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
