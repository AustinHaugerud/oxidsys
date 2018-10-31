use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopIsHeroOp;

const DOC : &str = "Checks the troop for tf_hero flag (see header_troops.py). Hero troops are actual characters and do not stack in party window.";

pub const OP_CODE: u32 = 1507;

pub const IDENT: &str = "troop_is_hero";

impl Operation for TroopIsHeroOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
