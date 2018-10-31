use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyGetPositionOp;

const DOC: &str = "Stores current position of the party on world map.";

pub const OP_CODE: u32 = 1625;

pub const IDENT: &str = "party_get_position";

impl Operation for PartyGetPositionOp {
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
                make_param_doc("<dest_position>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
