use language::operations::Operation;
pub mod game_key_get_mapped_key_name;
pub mod net;
pub mod str_clear;
pub mod str_store_agent_name;
pub mod str_store_class_name;
pub mod str_store_date;
pub mod str_store_faction_name;
pub mod str_store_faction_name_link;
pub mod str_store_info_page_name;
pub mod str_store_info_page_name_link;
pub mod str_store_item_name;
pub mod str_store_item_name_by_count;
pub mod str_store_item_name_plural;
pub mod str_store_party_name;
pub mod str_store_party_name_link;
pub mod str_store_quest_name;
pub mod str_store_quest_name_link;
pub mod str_store_string;
pub mod str_store_string_reg;
pub mod str_store_troop_name;
pub mod str_store_troop_name_by_count;
pub mod str_store_troop_name_link;
pub mod str_store_troop_name_plural;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        game_key_get_mapped_key_name::GameKeyGetMappedKeyNameOp {},
    ));
    result.push(Box::new(str_clear::StrClearOp {}));
    result.push(Box::new(str_store_agent_name::StrStoreAgentNameOp {}));
    result.push(Box::new(str_store_class_name::StrStoreClassNameOp {}));
    result.push(Box::new(str_store_date::StrStoreDateOp {}));
    result.push(Box::new(str_store_faction_name::StrStoreFactionNameOp {}));
    result.push(Box::new(
        str_store_faction_name_link::StrStoreFactionNameLinkOp {},
    ));
    result.push(Box::new(
        str_store_info_page_name::StrStoreInfoPageNameOp {},
    ));
    result.push(Box::new(
        str_store_info_page_name_link::StrStoreInfoPageNameLinkOp {},
    ));
    result.push(Box::new(str_store_item_name::StrStoreItemNameOp {}));
    result.push(Box::new(
        str_store_item_name_by_count::StrStoreItemNameByCountOp {},
    ));
    result.push(Box::new(
        str_store_item_name_plural::StrStoreItemNamePluralOp {},
    ));
    result.push(Box::new(str_store_party_name::StrStorePartyNameOp {}));
    result.push(Box::new(
        str_store_party_name_link::StrStorePartyNameLinkOp {},
    ));
    result.push(Box::new(str_store_quest_name::StrStoreQuestNameOp {}));
    result.push(Box::new(
        str_store_quest_name_link::StrStoreQuestNameLinkOp {},
    ));
    result.push(Box::new(str_store_string::StrStoreStringOp {}));
    result.push(Box::new(str_store_string_reg::StrStoreStringRegOp {}));
    result.push(Box::new(str_store_troop_name::StrStoreTroopNameOp {}));
    result.push(Box::new(
        str_store_troop_name_by_count::StrStoreTroopNameByCountOp {},
    ));
    result.push(Box::new(
        str_store_troop_name_link::StrStoreTroopNameLinkOp {},
    ));
    result.push(Box::new(
        str_store_troop_name_plural::StrStoreTroopNamePluralOp {},
    ));
    result.extend(net::load_operands());
    result
}
