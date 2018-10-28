use language::operations::Operation;

pub struct CurTableauAddOverrideItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1999;

pub const IDENT: &str = "cur_tableau_add_override_item";

impl Operation for CurTableauAddOverrideItemOp {
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
