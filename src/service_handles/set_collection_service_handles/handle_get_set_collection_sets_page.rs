use bson::{doc, Document};
use service_common_handles::UnaryResponseResult;
use tonic::async_trait;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};

use managers::traits::ManagerTrait;
use view::{add_query_filters, get_manage_schema_view};

use crate::{
    ids_codes::{
        field_ids::{SETS_ASSOCIATED_COLLECTIONS_FIELD_ID},
        manage_ids::SETS_MANAGE_ID,
    },
    protocols::*,
    
};

#[async_trait]
pub trait HandleGetSetCollectionSetsPage {
    /// 取得产品分页
    async fn handle_get_set_collection_sets_page(
        &self,
        request: Request<GetSetCollectionSetsPageRequest>,
    ) -> UnaryResponseResult<GetSetCollectionSetsPageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let collection_id = &request.get_ref().collection_id;
        let page_index = &request.get_ref().page_index;

        let manage_id = SETS_MANAGE_ID;

        // 管理可读性检查
        if !view::can_manage_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        // 可读性过滤, 没有设置过滤即不可读
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
            get_manage_schema_view(&account_id, &role_group, &manage_id.to_string(), &fields).await;
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
        let index = if * page_index == 0u32 {
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
}
