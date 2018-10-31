use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetRelationOp;

const DOC: &str = "Sets relation between two factions. Relation is in -100..100 range.";

pub const OP_CODE: u32 = 1270;

pub const IDENT: &str = "set_relation";

impl Operation for SetRelationOp {
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
                make_param_doc("<faction_id_1>", ""),
                make_param_doc("<faction_id_2>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
