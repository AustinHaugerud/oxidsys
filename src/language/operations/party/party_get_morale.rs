use language::operations::Operation;

pub struct PartyGetMoraleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1671;

pub const IDENT: &str = "party_get_morale";

impl Operation for PartyGetMoraleOp {
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
