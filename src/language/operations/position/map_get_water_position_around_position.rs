use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MapGetWaterPositionAroundPositionOp;

const DOC : &str = "Returns a random position on the global map in the vicinity of the source_position. Will always return a water position (i.e. sea, lake or river).";

pub const OP_CODE: u32 = 1629;

pub const IDENT: &str = "map_get_water_position_around_position";

impl Operation for MapGetWaterPositionAroundPositionOp {
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
