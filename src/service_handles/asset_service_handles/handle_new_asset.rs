use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::{
    DESCRIPTION_FIELD_ID, ID_FIELD_ID, NAME_MAP_FIELD_ID,
};
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use service_utils::validate_name;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleNewAsset {
    /// 新建
    async fn handle_new_asset(
        &self,
        request: Request<NewAssetRequest>,
    ) -> UnaryResponseResult<NewAssetResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_asset)
            .await
    }
}


async fn validate_view_rules(
    request: Request<NewAssetRequest>,
) -> Result<Request<NewAssetRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = ASSETS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewAssetRequest>,
) -> Result<Request<NewAssetRequest>, Status> {
    Ok(request)
}

async fn handle_new_asset(
    request: Request<NewAssetRequest>,
) -> Result<Response<NewAssetResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let asset_collection_id = &request.get_ref().asset_collection_id;
    let name = &request.get_ref().name;
    let description = &request.get_ref().description;

    if !validate_name(name){
        return Err(Status::data_loss("名字不能为空."));
    }
    let name = name.as_ref().unwrap();

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(ASSETS_MANAGE_ID)
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
        ASSETS_ORIGINAL_COLLECTION_FIELD_ID.to_string(),
        asset_collection_id.clone(),
    );
    new_entity_doc.insert(
        ASSETS_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        asset_collection_id.clone(),
    );
    new_entity_doc.insert(
        DESCRIPTION_FIELD_ID.to_string(),
        description.clone(),
    );

    let result = manager
        .sink_entity(&mut new_entity_doc, &account_id, &role_group)
        .await;

    match result {
        Ok(r) => Ok(Response::new(NewAssetResponse {
            //TODO: 发出新资产事件
            result: r,
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}