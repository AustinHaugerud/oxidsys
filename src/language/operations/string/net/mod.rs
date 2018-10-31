use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod str_encode_url;
pub mod str_store_player_username;
pub mod str_store_server_name;
pub mod str_store_server_password;
pub mod str_store_welcome_message;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(str_encode_url::StrEncodeUrlOp {}));
    result.push(Box::new(
        str_store_player_username::StrStorePlayerUsernameOp {},
    ));
    result.push(Box::new(str_store_server_name::StrStoreServerNameOp {}));
    result.push(Box::new(
        str_store_server_password::StrStoreServerPasswordOp {},
    ));
    result.push(Box::new(
        str_store_welcome_message::StrStoreWelcomeMessageOp {},
    ));
    result
}
