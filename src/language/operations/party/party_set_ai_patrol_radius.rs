use language::operations::Operation;

pub struct PartySetAiPatrolRadiusOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1643;

pub const IDENT: &str = "party_set_ai_patrol_radius";

impl Operation for PartySetAiPatrolRadiusOp {
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
