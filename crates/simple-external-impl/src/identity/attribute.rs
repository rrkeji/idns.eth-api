use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{
    AttributeCreateRequest, AttributeEntity, AttributeUpdateRequest, ListAttributesResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct AttributeJsonEntity {
    #[serde(rename(deserialize = "_id"))]
    pub id: crate::utils::ObjectId,
    pub source_identity: String,
    pub name: String,
    pub attribute_type: String,
    pub description: String,
    pub create_at: u64,
}

pub struct ExternalApiAttribute {}

impl ExternalApiAttribute {
    /// 获取属性列表
    pub async fn attribute_list(source_identity: &String) -> Result<Vec<AttributeEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/attribute/list/{}", source_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<AttributeJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| AttributeEntity {
                    id: item.id.oid.clone(),
                    source_identity: item.source_identity.clone(),
                    name: item.name.clone(),
                    attribute_type: item.attribute_type.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_one_attribute(id: &String) -> Result<AttributeEntity> {
        //访问
        if let Ok(json_str) =
            crate::identity::identity_get_request(format!("/attribute/get/{}", id).as_str()).await
        {
            //
            let item: AttributeJsonEntity = serde_json::from_str(&json_str)?;
            Ok(AttributeEntity {
                id: item.id.oid.clone(),
                source_identity: item.source_identity.clone(),
                name: item.name.clone(),
                attribute_type: item.attribute_type.clone(),
                description: item.description.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建属性
    pub async fn create_attribute(request: &AttributeCreateRequest) -> Result<()> {
        let body = &json!({
            "source_identity": request.source_identity.clone(),
            "name": request.name.clone(),
            "attribute_type": request.attribute_type.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/attribute/create",
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
    pub async fn remove_attribute(id: &String) -> Result<()> {
        //访问
        if let Ok(_) =
            crate::identity::identity_get_request(format!("/attribute/remove/{}", id).as_str())
                .await
        {
            //
            Ok(())
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 更新属性
    pub async fn update_attribute(request: &AttributeUpdateRequest) -> Result<()> {
        let body = &json!({
            "id": request.id.clone(),
            "source_identity": request.source_identity.clone(),
            "name": request.name.clone(),
            "attribute_type": request.attribute_type.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/attribute/update",
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
