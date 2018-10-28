use language::operations::Operation;

pub struct CurTableauClearOverrideItemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1998;

pub const IDENT: &str = "cur_tableau_clear_override_items";

impl Operation for CurTableauClearOverrideItemsOp {
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
