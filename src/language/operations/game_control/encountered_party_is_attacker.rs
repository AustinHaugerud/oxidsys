use language::operations::Operation;

pub struct EncounteredPartyIsAttackerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 39;

pub const IDENT: &str = "encountered_party_is_attacker";

impl Operation for EncounteredPartyIsAttackerOp {
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
