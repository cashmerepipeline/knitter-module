use dependencies_sync::bson::{doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::field_ids::*;
use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleDeassociateAssetCollectionsFromProject {
    /// 取消关联项目资产集合，
    /// NOTE: 特化方法，可能需要特殊操作而暂时保留
    async fn handle_deassociate_asset_collections_from_project(
        &self,
        request: Request<DeassociateAssetCollectionsFromProjectRequest>,
    ) -> UnaryResponseResult<DeassociateAssetCollectionsFromProjectResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_deassociate_asset_collections_from_project)
            .await
    }
}

async fn validate_view_rules(
    request: Request<DeassociateAssetCollectionsFromProjectRequest>,
) -> Result<Request<DeassociateAssetCollectionsFromProjectRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PROJECTS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<DeassociateAssetCollectionsFromProjectRequest>,
) -> Result<Request<DeassociateAssetCollectionsFromProjectRequest>, Status> {
    Ok(request)
}

async fn handle_deassociate_asset_collections_from_project(
    request: Request<DeassociateAssetCollectionsFromProjectRequest>,
) -> Result<Response<DeassociateAssetCollectionsFromProjectResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let project_id = &request.get_ref().project_id;
    let collection_ids = &request.get_ref().collection_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(PROJECTS_MANAGE_ID).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():project_id.clone(),
    };
    let mut modify_doc = Document::new();
    modify_doc.insert(
        PROJECTS_ASSET_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$in":collection_ids.clone()},
    );

    let result = manager
        .remove_from_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(
            DeassociateAssetCollectionsFromProjectResponse {
                result: "ok".to_string(),
            },
        )),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
