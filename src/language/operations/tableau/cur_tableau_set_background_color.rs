use language::operations::Operation;

pub struct CurTableauSetBackgroundColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1985;

pub const IDENT: &str = "cur_tableau_set_background_color";

impl Operation for CurTableauSetBackgroundColorOp {
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
