use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetCurrentTerrainOp;

const DOC: &str = "Returns a value from header_terrain_types.py";

pub const OP_CODE: u32 = 1608;

pub const IDENT: &str = "party_get_current_terrain";

impl Operation for PartyGetCurrentTerrainOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
