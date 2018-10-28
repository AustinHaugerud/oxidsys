use language::operations::Operation;

pub struct CurTableauSetOverrideFlagsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1997;

pub const IDENT: &str = "cur_tableau_set_override_flags";

impl Operation for CurTableauSetOverrideFlagsOp {
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
