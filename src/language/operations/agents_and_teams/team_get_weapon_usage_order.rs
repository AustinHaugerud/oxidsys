use language::operations::Operation;

pub struct TeamGetWeaponUsageOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1787;

pub const IDENT: &str = "team_get_weapon_usage_order";

impl Operation for TeamGetWeaponUsageOrderOp {
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
