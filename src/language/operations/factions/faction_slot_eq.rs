use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionSlotEqOp;

const DOC: &str =
    "faction_slot_ge                 =  562   (faction_slot_ge, <faction_id>, <slot_no>, <value>),";

pub const OP_CODE: u32 = 542;

pub const IDENT: &str = "faction_slot_eq";

impl Operation for FactionSlotEqOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<faction_id>", ""),
                make_param_doc("<slot_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
