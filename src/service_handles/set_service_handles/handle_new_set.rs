use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::tonic::async_trait;
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::{
    DESCRIPTIONS_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID,
};
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use service_utils::validate_name;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleNewSet {
    /// 新建产品
    async fn handle_new_set(
        &self,
        request: Request<NewSetRequest>,
    ) -> UnaryResponseResult<NewSetResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_set)
            .await
    }
}


async fn validate_view_rules(
    request: Request<NewSetRequest>,
) -> Result<Request<NewSetRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = SETS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewSetRequest>,
) -> Result<Request<NewSetRequest>, Status> {
    Ok(request)
}

async fn handle_new_set(
    request: Request<NewSetRequest>,
) -> Result<Response<NewSetResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let name = &request.get_ref().name;
    let collection_id = &request.get_ref().set_collection_id;
    let description = &request.get_ref().description;

    if !validate_name(name){
        return Err(Status::data_loss("名字不能为空."));
    }
    let name = name.as_ref().unwrap();

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(SETS_MANAGE_ID)
        .unwrap();

    // 新建条目
    let new_id = manager.get_new_entity_id(&account_id).await.unwrap();
    let mut new_entity_doc = Document::new();
    new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
    new_entity_doc.insert(
        NAME_MAP_FIELD_ID.to_string(),
        doc! {name.language.clone():name.name.clone()},
    );
    new_entity_doc.insert(
        SETS_ORIGINAL_COLLECTION_FIELD_ID.to_string(),
        collection_id.clone(),
    );
    new_entity_doc.insert(
        SETS_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        collection_id.clone(),
    );
    new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.clone());

    let result = manager
        .sink_entity(&mut new_entity_doc, &account_id, &role_group)
        .await;

    match result {
        Ok(r) => Ok(Response::new(NewSetResponse {
            // TODO: 发送新建事件
            result: r,
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}