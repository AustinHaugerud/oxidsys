use language::operations::Operation;

pub struct TroopRaiseSkillOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1521;

pub const IDENT: &str = "troop_raise_skill";

impl Operation for TroopRaiseSkillOp {
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
