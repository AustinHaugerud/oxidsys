use language::operations::Operation;

pub struct TeamGiveOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1790;

pub const IDENT: &str = "team_give_order";

impl Operation for TeamGiveOrderOp {
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
