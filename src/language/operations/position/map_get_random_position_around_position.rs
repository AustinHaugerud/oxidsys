use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MapGetRandomPositionAroundPositionOp;

const DOC: &str =
    "Returns a random position on the global map in the vicinity of the source_position.";

pub const OP_CODE: u32 = 1627;

pub const IDENT: &str = "map_get_random_position_around_position";

impl Operation for MapGetRandomPositionAroundPositionOp {
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
                make_param_doc("<dest_position_no>", ""),
                make_param_doc("<source_position_no>", ""),
                make_param_doc("<radius>", ""),
            ],
        }
    }
}
