use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetTypeOp;

const DOC : &str = "Changes the troop skin. There are two skins in Native: male and female, so in effect this operation sets troop gender. However mods may declare other skins.";

pub const OP_CODE: u32 = 1505;

pub const IDENT: &str = "troop_set_type";

impl Operation for TroopSetTypeOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<gender>", ""),
            ],
        }
    }
}
