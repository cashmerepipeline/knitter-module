use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use managers::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;


use crate::{
    ids_codes::{
        field_ids::*,
        manage_ids::ASSETS_MANAGE_ID,
    },
    protocols::*,
};

#[async_trait]
pub trait HandleGetAssetCollectionAssemblyTotalCount {
    /// 取得资产数量
    async fn handle_get_asset_collection_assembly_total_count(
        &self,
        request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
    ) -> UnaryResponseResult<GetAssetCollectionAssemblyTotalCountResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_asset_collection_assembly_total_count)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
) -> Result<Request<GetAssetCollectionAssemblyTotalCountRequest>, Status> {
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
    request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
) -> Result<Request<GetAssetCollectionAssemblyTotalCountRequest>, Status> {
    Ok(request)
}

async fn handle_get_asset_collection_assembly_total_count(
    request: Request<GetAssetCollectionAssemblyTotalCountRequest>,
) -> UnaryResponseResult<GetAssetCollectionAssemblyTotalCountResponse> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let collection_id = &request.get_ref().collection_id;
    let manage_id = ASSETS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let mut filter_doc = doc! {};
    filter_doc.insert(
        ASSEMBLIES_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$in":[collection_id]},
    );

    let result = manager.get_entry_counts(filter_doc).await;

    match result {
        Ok(r) => Ok(Response::new(
            GetAssetCollectionAssemblyTotalCountResponse { total_count: r },
        )),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}