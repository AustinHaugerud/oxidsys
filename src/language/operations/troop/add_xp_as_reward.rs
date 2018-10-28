use language::operations::Operation;

pub struct AddXpAsRewardOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1064;

pub const IDENT: &str = "add_xp_as_reward";

impl Operation for AddXpAsRewardOp {
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
