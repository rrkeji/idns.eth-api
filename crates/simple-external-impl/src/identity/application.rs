use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{
    ApplicationCreateRequest, ApplicationEntity, ApplicationSearchRequest,
    ApplicationUpdateRequest, ListApplicationsResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct ApplicationJsonEntity {
    #[serde(rename(deserialize = "_id"))]
    pub id: crate::utils::ObjectId,
    pub provider: String,
    pub application_type: String,
    pub name: String,
    pub avatar: String,
    pub application_cid: String,
    pub labels: Vec<String>,
    pub signature: String,
    pub description: String,
    pub create_at: u64,
}

pub struct ExternalApiApplication {}

impl ExternalApiApplication {
    /// 获取属性列表
    pub async fn application_search(
        request: &ApplicationSearchRequest,
    ) -> Result<Vec<ApplicationEntity>> {
        // let body = &json!({
        //     "conditions": request.provider.clone(),
        //     "application_type": request.application_type.clone(),
        //     "name": request.name.clone(),
        //     "avatar": request.avatar.clone(),
        //     "application_cid": request.application_cid.clone(),
        //     "labels": request.labels.clone(),
        //     "signature": request.signature.clone(),
        //     "description":request.description.clone()
        // })
        // .to_string();

        // //访问
        // if let Ok(json_str) =
        //     crate::identity::identity_post_request("/application/search", body).await
        // {
        //     //
        //     let list: Vec<ApplicationJsonEntity> = serde_json::from_str(&json_str)?;
        //     Ok(list
        //         .iter()
        //         .map(|item| ApplicationEntity {
        //             id: item.id,
        //             provider: item.provider.clone(),
        //             application_type: item.application_type.clone(),
        //             name: item.name.clone(),
        //             avatar: item.avatar.clone(),
        //             application_cid: item.application_cid.clone(),
        //             labels: item.labels.clone(),
        //             signature: item.signature.clone(),
        //             description: item.description.clone(),
        //             create_at: item.create_at,
        //         })
        //         .collect())
        // } else {
        // }
        Ok(vec![])
    }
    /// 获取属性列表
    pub async fn application_mylist(provider: &String) -> Result<Vec<ApplicationEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/application/mylist/{}", provider).as_str(),
        )
        .await
        {
            tracing::debug!("application_mylist 返回:{}", json_str);
            //
            let list: Vec<ApplicationJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| ApplicationEntity {
                    id: item.id.oid.clone(),
                    provider: item.provider.clone(),
                    application_type: item.application_type.clone(),
                    name: item.name.clone(),
                    avatar: item.avatar.clone(),
                    application_cid: item.application_cid.clone(),
                    labels: item.labels.clone(),
                    signature: item.signature.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_one_application(id: &String) -> Result<ApplicationEntity> {
        //访问
        if let Ok(json_str) =
            crate::identity::identity_get_request(format!("/application/get/{}", id).as_str()).await
        {
            //
            let item: ApplicationJsonEntity = serde_json::from_str(&json_str)?;
            Ok(ApplicationEntity {
                id: item.id.oid.clone(),
                provider: item.provider.clone(),
                application_type: item.application_type.clone(),
                name: item.name.clone(),
                avatar: item.avatar.clone(),
                application_cid: item.application_cid.clone(),
                labels: item.labels.clone(),
                signature: item.signature.clone(),
                description: item.description.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建属性
    pub async fn create_application(request: &ApplicationCreateRequest) -> Result<()> {
        let body = &json!({
            "provider": request.provider.clone(),
            "application_type": request.application_type.clone(),
            "name": request.name.clone(),
            "avatar": request.avatar.clone(),
            "application_cid": request.application_cid.clone(),
            "labels": request.labels.clone(),
            "signature": request.signature.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/application/create",
            "application/json;charset=UTF-8",
            body,
        )
        .await
        .with_context(|| format!("创建属性请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建属性 {}!", body))
        }
    }
    /// 删除属性
    pub async fn remove_application(id: &String) -> Result<()> {
        //访问
        if let Ok(_) =
            crate::identity::identity_get_request(format!("/application/remove/{}", id).as_str())
                .await
        {
            //
            Ok(())
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 更新属性
    pub async fn update_application(request: &ApplicationUpdateRequest) -> Result<()> {
        let body = &json!({
            "id": request.id.clone(),
            "provider": request.provider.clone(),
            "application_type": request.application_type.clone(),
            "name": request.name.clone(),
            "avatar": request.avatar.clone(),
            "application_cid": request.application_cid.clone(),
            "labels": request.labels.clone(),
            "signature": request.signature.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/application/update",
            "application/json;charset=UTF-8",
            body,
        )
        .await
        .with_context(|| format!("创建属性请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建属性 {}!", body))
        }
    }
}
