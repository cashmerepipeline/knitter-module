use tonic::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use service_common_handles::UnaryResponseResult;
use tonic::{Request, Response, Status};

use crate::ids_codes::manage_ids::*;
use crate::protocols::*;
use manage_define::general_field_ids::ID_FIELD_ID;




#[async_trait]
pub trait HandleGetReferencedAssets {
    /// 新建产品
    async fn handle_get_referenced_assets(
        &self,
        request: Request<GetReferencedAssetsRequest>,
    ) -> UnaryResponseResult<GetReferencedAssetsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let _entity_id = &request.get_ref().entity_id;
        let asset_ids = &request.get_ref().asset_ids;

        if !view::can_entity_read(&account_id, &role_group, &manage_id.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有工程可读权限"));
        }
        if !view::can_collection_read(&account_id, &role_group, &ASSETS_MANAGE_ID.to_string()).await
        {
            return Err(Status::unauthenticated("用户不具有景可读权限"));
        }

        // TODO: 可能需要关联用户工程可读检查

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(ASSETS_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():doc! {"$in":asset_ids.clone()},
        };

        let result = manager.get_entities_by_filter(&Some(query_doc)).await;

        match result {
            Ok(r) => Ok(Response::new(GetReferencedAssetsResponse {
                assets: r.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


