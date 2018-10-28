use language::operations::Operation;

pub struct CurTableauSetAmbientLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1987;

pub const IDENT: &str = "cur_tableau_set_ambient_light";

impl Operation for CurTableauSetAmbientLightOp {
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
