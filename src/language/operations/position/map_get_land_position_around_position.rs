use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MapGetLandPositionAroundPositionOp;

const DOC : &str = "Returns a random position on the global map in the vicinity of the source_position. Will always return a land position (i.e. some place you can walk to).";

pub const OP_CODE: u32 = 1628;

pub const IDENT: &str = "map_get_land_position_around_position";

impl Operation for MapGetLandPositionAroundPositionOp {
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
