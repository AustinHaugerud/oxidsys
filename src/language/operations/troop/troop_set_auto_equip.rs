use language::operations::Operation;

pub struct TroopSetAutoEquipOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1509;

pub const IDENT: &str = "troop_set_auto_equip";

impl Operation for TroopSetAutoEquipOp {
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
