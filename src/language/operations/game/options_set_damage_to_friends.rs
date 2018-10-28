use language::operations::Operation;

pub struct OptionsSetDamageToFriendsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 263;

pub const IDENT: &str = "options_set_damage_to_friends";

impl Operation for OptionsSetDamageToFriendsOp {
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
