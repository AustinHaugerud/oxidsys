use language::operations::Operation;

pub struct PartySetIgnoreWithPlayerPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1648;

pub const IDENT: &str = "party_set_ignore_with_player_party";

impl Operation for PartySetIgnoreWithPlayerPartyOp {
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
