use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::tonic::async_trait;
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use view::{add_query_filters, get_manage_schema_view};

use crate::{
    ids_codes::{field_ids::SETS_ASSOCIATED_COLLECTIONS_FIELD_ID, manage_ids::SETS_MANAGE_ID},
    protocols::*,
};

#[async_trait]
pub trait HandleGetSetCollectionSetsPage {
    /// 取得产品分页
    async fn handle_get_set_collection_sets_page(
        &self,
        request: Request<GetSetCollectionSetsPageRequest>,
    ) -> UnaryResponseResult<GetSetCollectionSetsPageResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_set_collection_sets_page)
            .await
    }
}


async fn validate_view_rules(
    request: Request<GetSetCollectionSetsPageRequest>,
) -> Result<Request<GetSetCollectionSetsPageRequest>, Status> {
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
    request: Request<GetSetCollectionSetsPageRequest>,
) -> Result<Request<GetSetCollectionSetsPageRequest>, Status> {
    Ok(request)
}

async fn handle_get_set_collection_sets_page(
    request: Request<GetSetCollectionSetsPageRequest>,
) -> Result<Response<GetSetCollectionSetsPageResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let collection_id = &request.get_ref().collection_id;
    let page_index = &request.get_ref().page_index;

    let manage_id = SETS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    // TODO: 根据组改写，加入可读过滤项
    let mut matches = doc! {};
    if let Some(filter_doc) =
        add_query_filters(&account_id.to_string(), &role_group, &manage_id.to_string()).await
    {
        filter_doc.iter().for_each(|(k, v)| {
            matches.insert(k, v);
        });
    } else {
        return Err(Status::unauthenticated(
            "没有可读描写字段，用户不具有集合可读权限",
        ));
    };

    let mut matches = doc! {};
    matches.insert(
        SETS_ASSOCIATED_COLLECTIONS_FIELD_ID.to_string(),
        doc! {"$in":vec![collection_id.clone()]},
    );

    // zh: 描写字段可见性过滤, 加入mongodb的project方法
    let fields = manager.get_manage_schema().await;
    let schema_projects =
        get_manage_schema_view(&manage_id.to_string(), &fields, &role_group).await;
    let project_doc = if !schema_projects.is_empty() {
        // 只加入不可见字段
        let mut no_show_project = Document::new();
        schema_projects.iter().for_each(|(k, v)| {
            if v.as_i32().unwrap() == 0 {
                no_show_project.insert(k, v);
            }
        });
        Some(no_show_project)
    } else {
        None
    };
    // zh: 从1开始，
    let index = if *page_index == 0u32 {
        1u32
    } else {
        *page_index
    };

    let result = manager
        .get_entities_by_page(index, &Some(matches), &None, &project_doc)
        .await;

    match result {
        Ok(entities) => Ok(Response::new(GetSetCollectionSetsPageResponse {
            sets: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}