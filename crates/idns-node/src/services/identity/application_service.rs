use bytes::Bytes;
use idns_eth_api::idns::identity::{
    ApplicationCreateRequest, ApplicationEntity, ApplicationUpdateRequest, ListApplicationsResponse,
};
use idns_eth_api::idns::system::{BoolMessage, StringMessage, StringPairMessage, U64Message};
use idns_eth_api::{response, Command, CommandResponse, Error, Handler, Result};
use prost::Message;

pub struct ApplicationServiceImpl {}

impl ApplicationServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplicationServiceImpl {
    ///
    pub async fn application_mylist(&self, provider: &String) -> Result<Vec<ApplicationEntity>> {
        let res = idns_eth_core::identity::IdnsApplication::application_mylist(provider).await?;
        Ok(res)
    }

    pub async fn query_one_application(&self, id: &String) -> Result<ApplicationEntity> {
        let res = idns_eth_core::identity::IdnsApplication::query_one_application(id).await?;
        Ok(res)
    }

    pub async fn create_application(&self, request: &ApplicationCreateRequest) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsApplication::create_application(request).await?;
        Ok(true)
    }
    pub async fn remove_application(&self, id: &String) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsApplication::remove_application(id).await?;
        Ok(true)
    }
    pub async fn update_application(&self, request: &ApplicationUpdateRequest) -> Result<bool> {
        let _ = idns_eth_core::identity::IdnsApplication::update_application(request).await?;
        Ok(true)
    }

    pub async fn is_cached(&self, id: &String) -> Result<bool> {
        //判断目录是否存在
        let storage_path = idns_eth_core::idns_home_path()?
            .join("webroot/apps")
            .join(id.replace(":", "_"));

        Ok(storage_path.as_path().exists())
    }
    pub async fn cache(&self, id: &String) -> Result<bool> {
        let cached = self.is_cached(id).await?;
        if cached {
            return Ok(true);
        }
        let application = self.query_one_application(id).await?;
        crate::http::Server::download(&application).await?;

        Ok(true)
    }

    pub async fn get_view_url(&self, id: &String) -> Result<String> {
        let application = self.query_one_application(id).await?;
        //TODO 应用的类型不同主目录不同
        let root = crate::http::Server::get_root_uri()?;

        Ok(format!("{}/apps/{}/index.html", root, id))
    }
}

#[async_trait::async_trait]
impl Handler for ApplicationServiceImpl {
    async fn execute(&self, request: Command) -> Result<CommandResponse> {
        let service_name = request.service_name;
        let method_name = request.method_name;
        let message = request.data;

        if service_name == "idns.system.identity.application" {
            if method_name == "application_mylist" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.application_mylist(&request.data)
                        .await
                        .map(|r| ListApplicationsResponse { applications: r }),
                );
            } else if method_name == "query_one_application" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(self.query_one_application(&request.data).await);
            } else if method_name == "is_cached" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.is_cached(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "cache" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.cache(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "get_view_url" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.get_view_url(&request.data)
                        .await
                        .map(|r| StringMessage { data: r }),
                );
            } else if method_name == "remove_application" {
                let request = StringMessage::decode(Bytes::from(message))?;
                //
                return response(
                    self.remove_application(&request.data)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "create_application" {
                let request = ApplicationCreateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.create_application(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            } else if method_name == "update_application" {
                let request = ApplicationUpdateRequest::decode(Bytes::from(message))?;
                //
                return response(
                    self.update_application(&request)
                        .await
                        .map(|r| BoolMessage { data: r }),
                );
            }
        }
        Err(Error::NotFoundService)
    }
}
