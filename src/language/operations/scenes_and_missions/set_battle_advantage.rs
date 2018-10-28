use language::operations::Operation;

pub struct SetBattleAdvantageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1691;

pub const IDENT: &str = "set_battle_advantage";

impl Operation for SetBattleAdvantageOp {
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
