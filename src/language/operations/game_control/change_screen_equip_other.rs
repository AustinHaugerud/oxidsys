use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ChangeScreenEquipOtherOp;

const DOC : &str = "Opens the Equip Companion interface. When calling from a dialog, it is not necessary to specify troop_id.";

pub const OP_CODE: u32 = 2051;

pub const IDENT: &str = "change_screen_equip_other";

impl Operation for ChangeScreenEquipOtherOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 0,
            num_optional: 1,
            param_docs: vec![make_param_doc("[troop_id]", "")],
        }
    }
}
