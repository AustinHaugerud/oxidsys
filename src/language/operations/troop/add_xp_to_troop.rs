use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddXpToTroopOp;

const DOC : &str = "Adds some xp points to troop. Only makes sense for player and hero troops. Default troop_id is player. Amount of xp can be negative.";

pub const OP_CODE: u32 = 1062;

pub const IDENT: &str = "add_xp_to_troop";

impl Operation for AddXpToTroopOp {
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
            num_required: 1,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<value>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}
