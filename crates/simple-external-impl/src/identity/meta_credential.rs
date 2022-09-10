use anyhow::{anyhow, Context, Result};
use idns_eth_api::idns::identity::{
    ListMetaCredentialsResponse, MetaCredentialCreateRequest, MetaCredentialEntity,
    MetaCredentialUpdateRequest,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
struct MetaCredentialJsonEntity {
    #[serde(rename(deserialize = "_id"))]
    pub id: crate::utils::ObjectId,
    pub source_identity: String,
    pub name: String,
    pub credential_type: String,
    pub expires: u32,
    pub issuance_date: u32,
    pub issuance_status: u32,
    pub assert_expression: String,
    pub parameters: String,
    pub description: String,
    pub create_at: u64,
}

pub struct ExternalApiMetaCredential {}

impl ExternalApiMetaCredential {
    /// 获取元证书列表
    pub async fn meta_credential_list(
        source_identity: &String,
    ) -> Result<Vec<MetaCredentialEntity>> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/meta_credential/list/{}", source_identity).as_str(),
        )
        .await
        {
            //
            let list: Vec<MetaCredentialJsonEntity> = serde_json::from_str(&json_str)?;
            Ok(list
                .iter()
                .map(|item| MetaCredentialEntity {
                    id: item.id.oid.clone(),
                    source_identity: item.source_identity.clone(),
                    name: item.name.clone(),
                    credential_type: item.credential_type.clone(),
                    expires: item.expires,
                    issuance_date: item.issuance_date,
                    issuance_status: item.issuance_status,
                    assert_expression: item.assert_expression.clone(),
                    parameters: item.parameters.clone(),
                    description: item.description.clone(),
                    create_at: item.create_at,
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }
    /// 查询一个
    pub async fn query_one_meta_credential(
        source_identity: &String,
        name: &String,
    ) -> Result<MetaCredentialEntity> {
        //访问
        if let Ok(json_str) = crate::identity::identity_get_request(
            format!("/meta_credential/get/{}/{}", source_identity, name).as_str(),
        )
        .await
        {
            //
            let item: MetaCredentialJsonEntity = serde_json::from_str(&json_str)?;
            Ok(MetaCredentialEntity {
                id: item.id.oid.clone(),
                source_identity: item.source_identity.clone(),
                name: item.name.clone(),
                credential_type: item.credential_type.clone(),
                expires: item.expires,
                issuance_date: item.issuance_date,
                issuance_status: item.issuance_status,
                assert_expression: item.assert_expression.clone(),
                parameters: item.parameters.clone(),
                description: item.description.clone(),
                create_at: item.create_at,
            })
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 创建元证书
    pub async fn create_meta_credential(request: &MetaCredentialCreateRequest) -> Result<()> {
        let body = &json!({
            "source_identity": request.source_identity.clone(),
            "name": request.name.clone(),
            "credential_type": request.credential_type.clone(),
            "expires":request.expires,
            "assert_expression":request.assert_expression.clone(),
            "parameters":request.parameters.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/meta_credential/create",
            "application/json;charset=UTF-8",
            body,
        )
        .await
        .with_context(|| format!("创建元证书请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建元证书 {}!", body))
        }
    }
    /// 删除元证书
    pub async fn remove_meta_credential(source_identity: &String, name: &String) -> Result<()> {
        //访问
        if let Ok(_) = crate::identity::identity_get_request(
            format!("/meta_credential/remove/{}/{}", source_identity, name).as_str(),
        )
        .await
        {
            //
            Ok(())
        } else {
            Err(anyhow!("没有查询到相关的数据"))
        }
    }
    /// 更新元证书
    pub async fn update_meta_credential(request: &MetaCredentialUpdateRequest) -> Result<()> {
        let body = &json!({
            "source_identity": request.source_identity.clone(),
            "name": request.name.clone(),
            "credential_type": request.credential_type.clone(),
            "expires":request.expires,
            "assert_expression":request.assert_expression.clone(),
            "parameters":request.parameters.clone(),
            "description":request.description.clone()
        })
        .to_string();

        //
        if let Ok(res) = crate::identity::identity_post_request(
            "/meta_credential/update",
            "application/json;charset=UTF-8",
            body,
        )
        .await
        .with_context(|| format!("创建元证书请求失败 {}!", body))
        {
            tracing::debug!("{}", res);

            // 空判断
            Ok(())
        } else {
            Err(anyhow!("创建元证书 {}!", body))
        }
    }
}
