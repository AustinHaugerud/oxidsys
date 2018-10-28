use language::operations::Operation;

pub struct CurTableauAddMapIconOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1994;

pub const IDENT: &str = "cur_tableau_add_map_icon";

impl Operation for CurTableauAddMapIconOp {
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
