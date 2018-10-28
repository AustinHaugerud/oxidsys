use language::operations::Operation;

pub struct ChangeScreenEquipOtherOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2051;

pub const IDENT: &str = "change_screen_equip_other";

impl Operation for ChangeScreenEquipOtherOp {
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
