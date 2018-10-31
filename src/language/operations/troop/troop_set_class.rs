use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetClassOp;

const DOC : &str = "Sets troop class (infantry, archers, cavalry or any of custom classes). Accepts values in range 0..8. See grc_* constants in header_mission_templates.py.";

pub const OP_CODE: u32 = 1517;

pub const IDENT: &str = "troop_set_class";

impl Operation for TroopSetClassOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
