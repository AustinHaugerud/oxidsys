use language::operations::Operation;

pub struct CurTableauAddSunLightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1991;

pub const IDENT: &str = "cur_tableau_add_sun_light";

impl Operation for CurTableauAddSunLightOp {
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
