use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{
    AttributeValueCreateRequest, AttributeValueEntity, ListAttributeValuesResponse,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct AttributeValueJsonEntity {
    #[serde(rename(deserialize = "_id"))]
    pub id: crate::utils::ObjectId,
    pub source_identity: String,
    pub holder_identity: String,
    pub name: String,
    pub value: String,
    pub labels: String,
    pub description: String,
    pub create_at: u64,
}

pub struct ExternalApiAttributeValue {}

impl ExternalApiAttributeValue {
    /// 获取属性值列表
    pub async fn attribute_value_list_by_source(
        source_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/attribute_value/listbysource/{}", source_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<AttributeValueJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| AttributeValueEntity {
                    id: item.id.oid.clone(),
                    source_identity: item.source_identity.clone(),
                    holder_identity: item.holder_identity.clone(),
                    name: item.name.clone(),
                    value: item.value.clone(),
                    labels: item.labels.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 获取属性值列表
    pub async fn attribute_value_list_by_holder(
        holder_identity: &String,
    ) -> Result<Vec<AttributeValueEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/attribute_value/listbyholder/{}", holder_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<AttributeValueJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| AttributeValueEntity {
                    id: item.id.oid.clone(),
                    source_identity: item.source_identity.clone(),
                    holder_identity: item.holder_identity.clone(),
                    name: item.name.clone(),
                    value: item.value.clone(),
                    labels: item.labels.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_one_attribute_value(id: &String) -> Result<AttributeValueEntity> {
        //访问
        if let Ok(json_str) =
            crate::identity::identity_get_request(format!("/attribute_value/get/{}", id).as_str())
                .await
        {
            //
            let item: AttributeValueJsonEntity = serde_json::from_str(&json_str)?;
            Ok(AttributeValueEntity {
                id: item.id.oid.clone(),
                source_identity: item.source_identity.clone(),
                holder_identity: item.holder_identity.clone(),
                name: item.name.clone(),
                value: item.value.clone(),
                labels: item.labels.clone(),
                description: item.description.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建属性值
    pub async fn create_attribute_value(request: &AttributeValueCreateRequest) -> Result<()> {
        //
        let body = &json!({
            "source_identity": request.source_identity.clone(),
            "holder_identity": request.holder_identity.clone(),
            "name": request.name.clone(),
            "value": request.value.clone(),
            "labels":request.labels.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/attribute_value/create",
            "application/json;charset=UTF-8",
            body,
        )
        .await
        .with_context(|| format!("创建属性值请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建属性值 {}!", body))
        }
    }
    /// 删除属性值
    pub async fn remove_attribute_value(id: &String) -> Result<()> {
        //访问
        if let Ok(_) = crate::identity::identity_get_request(
            format!("/attribute_value/remove/{}", id).as_str(),
        )
        .await
        {
            //
            Ok(())
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
}
