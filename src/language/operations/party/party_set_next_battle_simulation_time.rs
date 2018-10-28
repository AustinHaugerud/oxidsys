use language::operations::Operation;

pub struct PartySetNextBattleSimulationTimeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1667;

pub const IDENT: &str = "party_set_next_battle_simulation_time";

impl Operation for PartySetNextBattleSimulationTimeOp {
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
