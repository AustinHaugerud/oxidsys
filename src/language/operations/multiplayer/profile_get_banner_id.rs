use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ProfileGetBannerIdOp;

const DOC : &str = "Client operation. Retrieves banner_id reference used by the game for multiplayer. Note that in MP banners are enumerated starting from 0 (unlike single-player where they're enumeration depends on scene prop banners' reference range).";

pub const OP_CODE: u32 = 350;

pub const IDENT: &str = "profile_get_banner_id";

impl Operation for ProfileGetBannerIdOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}
