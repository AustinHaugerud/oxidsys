use language::operations::Operation;

pub struct PartyIsInTownOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 130;

pub const IDENT: &str = "party_is_in_town";

impl Operation for PartyIsInTownOp {
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
