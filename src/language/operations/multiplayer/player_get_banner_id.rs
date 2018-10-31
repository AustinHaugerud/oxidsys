use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerGetBannerIdOp;

const DOC : &str = "Server operation. Retrieves banner_id reference used by the specified player. Note that in MP banners are enumerated starting from 0 (unlike single-player where they're enumeration depends on scene prop banners' reference range).";

pub const OP_CODE: u32 = 423;

pub const IDENT: &str = "player_get_banner_id";

impl Operation for PlayerGetBannerIdOp {
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
                make_param_doc("<player_id>", ""),
            ],
        }
    }
}
