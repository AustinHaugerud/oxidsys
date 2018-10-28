use language::operations::Operation;

pub struct CurTableauAddPointLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1990;

pub const IDENT: &str = "cur_tableau_add_point_light";

impl Operation for CurTableauAddPointLightOp {
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
