use language::operations::Operation;

pub struct PartyIsInAnyTownOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 131;

pub const IDENT: &str = "party_is_in_any_town";

impl Operation for PartyIsInAnyTownOp {
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
