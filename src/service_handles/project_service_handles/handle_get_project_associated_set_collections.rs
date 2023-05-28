use dependencies_sync::bson::{self, doc};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use service_utils::types::UnaryResponseResult;

use crate::ids_codes::manage_ids::*;
use crate::protocols::*;

#[async_trait]
pub trait HandleGetProjectAssociatedSetCollections {
    /// 新建产品
    async fn handle_get_project_associated_set_collections(
        &self,
        request: Request<GetProjectAssociatedSetCollectionsRequest>,
    ) -> UnaryResponseResult<GetProjectAssociatedSetCollectionsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let _project_id = &request.get_ref().project_id;
        let set_collection_ids = &request.get_ref().collection_ids;

        // TODO: 可能需要关联用户工程可读检查
        // TODO: 需要检查工程是有关联，列出漏洞

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(SET_COLLECTIONS_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():doc! {"$in":set_collection_ids.clone()},
        };

        let result = manager.get_entities_by_filter(&Some(query_doc)).await;

        match result {
            Ok(r) => Ok(Response::new(GetProjectAssociatedSetCollectionsResponse {
                set_collections: r.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


async fn validate_view_rules(
    request: Request<GetProjectAssociatedSetCollectionsRequest>,
) -> Result<Request<GetProjectAssociatedSetCollectionsRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        // TODO: 需要检查工程是有关联

        let manage_id = SET_COLLECTIONS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetProjectAssociatedSetCollectionsRequest>,
) -> Result<Request<GetProjectAssociatedSetCollectionsRequest>, Status> {
    Ok(request)
}