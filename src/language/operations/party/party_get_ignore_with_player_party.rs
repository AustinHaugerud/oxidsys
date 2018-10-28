use language::operations::Operation;

pub struct PartyGetIgnoreWithPlayerPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1649;

pub const IDENT: &str = "party_get_ignore_with_player_party";

impl Operation for PartyGetIgnoreWithPlayerPartyOp {
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
