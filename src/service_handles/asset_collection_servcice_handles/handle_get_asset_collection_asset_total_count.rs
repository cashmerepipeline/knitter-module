use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;


use crate::{
    ids_codes::{
        field_ids::ASSETS_ASSOCIATED_COLLECTIONS_FIELD_ID,
        manage_ids::ASSETS_MANAGE_ID,
    },
    protocols::*,
};

#[async_trait]
pub trait HandleGetAssetCollectionAssetTotalCount {
    /// 取得资产数量
    async fn handle_get_asset_collection_asset_total_count(
        &self,
        request: Request<GetAssetCollectionAssetTotalCountRequest>,
    ) -> UnaryResponseResult<GetAssetCollectionAssetTotalCountResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_asset_collection_asset_total_count)
            .await
    }
}


async fn validate_view_rules(
    request: Request<GetAssetCollectionAssetTotalCountRequest>,
) -> Result<Request<GetAssetCollectionAssetTotalCountRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        use request_utils::request_account_context;

        let manage_id = ASSEMBLIES_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetAssetCollectionAssetTotalCountRequest>,
) -> Result<Request<GetAssetCollectionAssetTotalCountRequest>, Status> {
    Ok(request)
}

async fn handle_get_asset_collection_asset_total_count(
    request: Request<GetAssetCollectionAssetTotalCountRequest>,
) -> UnaryResponseResult<GetAssetCollectionAssetTotalCountResponse> {
    let collection_id = &request.get_ref().collection_id;
    let manage_id = ASSETS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let mut filter_doc = doc! {};
    filter_doc.insert(
        ASSETS_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$in":[collection_id]},
    );

    let result = manager.get_entry_counts(filter_doc).await;

    match result {
        Ok(r) => Ok(Response::new(GetAssetCollectionAssetTotalCountResponse {
            total_count: r,
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}