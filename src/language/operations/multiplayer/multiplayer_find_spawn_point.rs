use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MultiplayerFindSpawnPointOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 425;

pub const IDENT: &str = "multiplayer_find_spawn_point";

impl Operation for MultiplayerFindSpawnPointOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<team_no>", ""),
                make_param_doc("<examine_all_spawn_points>", ""),
                make_param_doc("<is_horseman>", ""),
            ],
        }
    }
}
