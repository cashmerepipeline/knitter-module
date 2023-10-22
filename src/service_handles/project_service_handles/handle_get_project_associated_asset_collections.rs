use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleGetProjectAssociatedAssetCollections {
    /// 新建产品
    async fn handle_get_project_associated_asset_collections(
        &self,
        request: Request<GetProjectAssociatedAssetCollectionsRequest>,
    ) -> UnaryResponseResult<GetProjectAssociatedAssetCollectionsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_project_associated_asset_collections)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetProjectAssociatedAssetCollectionsRequest>,
) -> Result<Request<GetProjectAssociatedAssetCollectionsRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        // TODO: 可能需要关联用户工程可读检查

        let manage_id = ASSET_COLLECTIONS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetProjectAssociatedAssetCollectionsRequest>,
) -> Result<Request<GetProjectAssociatedAssetCollectionsRequest>, Status> {
    Ok(request)
}

async fn handle_get_project_associated_asset_collections(
    request: Request<GetProjectAssociatedAssetCollectionsRequest>,
) -> Result<Response<GetProjectAssociatedAssetCollectionsResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let _project_id = &request.get_ref().project_id;
    let library_ids = &request.get_ref().collection_ids;

    // TODO: 可能需要关联用户工程可读检查

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(ASSET_COLLECTIONS_MANAGE_ID)
        .unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():doc! {"$in":library_ids.clone()},
    };

    let result = manager.get_entities_by_filter(&Some(query_doc)).await;

    match result {
        Ok(r) => Ok(Response::new(
            GetProjectAssociatedAssetCollectionsResponse {
                asset_collections: r.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            },
        )),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
