use language::operations::Operation;

pub struct MissionTplEntryAddOverrideItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1942;

pub const IDENT: &str = "mission_tpl_entry_add_override_item";

impl Operation for MissionTplEntryAddOverrideItemOp {
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
