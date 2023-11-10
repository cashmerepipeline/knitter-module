use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::tonic::async_trait;
use majordomo::{self, get_majordomo};
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;


use crate::{
    ids_codes::{field_ids::SETS_ASSOCIATED_COLLECTIONS_FIELD_ID, manage_ids::SETS_MANAGE_ID},
    protocols::*,
};

#[async_trait]
pub trait HandleGetSetCollectionSetTotalCount {
    /// 取得资产数量
    async fn handle_get_set_collection_set_total_count(
        &self,
        request: Request<GetSetCollectionSetTotalCountRequest>,
    ) -> UnaryResponseResult<GetSetCollectionSetTotalCountResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_set_collection_set_total_count)
            .await
    }
}


async fn validate_view_rules(
    request: Request<GetSetCollectionSetTotalCountRequest>,
) -> Result<Request<GetSetCollectionSetTotalCountRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = SETS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetSetCollectionSetTotalCountRequest>,
) -> Result<Request<GetSetCollectionSetTotalCountRequest>, Status> {
    Ok(request)
}

async fn handle_get_set_collection_set_total_count(
    request: Request<GetSetCollectionSetTotalCountRequest>,
) -> Result<Response<GetSetCollectionSetTotalCountResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let collection_id = &request.get_ref().collection_id;
    let manage_id = SETS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let mut filter_doc = doc! {};
    filter_doc.insert(
        SETS_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$in":[collection_id]},
    );

    let result = manager.count_entity(filter_doc).await;

    match result {
        Ok(r) => Ok(Response::new(GetSetCollectionSetTotalCountResponse {
            total_count: r,
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}