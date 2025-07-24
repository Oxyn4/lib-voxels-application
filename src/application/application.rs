use std::time::Duration;
use dbus_tokio::connection::IOResourceError;
use super::rdn::{
    ApplicationRDNErrors,
    ApplicationRDN
};

use serde::{Serialize, Deserialize};

use thiserror::Error;

use uuid::Uuid;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ApplicationErrors {
    #[error("Invalid RDN provided for application")]
    InvalidRDN(ApplicationRDNErrors),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationsType {
    Client,
    Server,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    rdn: ApplicationRDN,
    id: Uuid,
    homepage: Option<Url>,
    description: Option<String>,
    app_type: Option<ApplicationsType>
}

impl Application {
    pub fn new(rdn: ApplicationRDN, id: Uuid, homepage: Option<url::Url>, description: Option<String>, app_type: Option<ApplicationsType>) -> Application {
        Application {
            rdn,
            id,
            homepage,
            description,
            app_type
        }
    }

    #[cfg(feature = "dbus")]
    pub async fn from_dbus<F>(uuid: Uuid, on_connection_loss: F) -> Result<Application, dbus::Error>
    where
        F: FnOnce(IOResourceError) + Send + 'static
    {
        let (res, con) = dbus_tokio::connection::new_system_sync()?;

        let cancel_token = tokio_util::sync::CancellationToken::new();

        let child_token = cancel_token.child_token();

        let _ = tokio::spawn(async move {
            tokio::select! {
                err = res => {
                    on_connection_loss(err);
                },
                _ = child_token.cancelled() => {
                    return;
                }
            }
        });

        let proxy = dbus::nonblock::Proxy::new("voxels.applications", "/get", Duration::from_secs(2), con);

        let (rdn,): (String,) = proxy.method_call("voxels.applications", "rdn", (uuid.to_string(),)).await?;
        let (description,): (String,) = proxy.method_call("voxels.applications", "description", (uuid.to_string(),)).await?;
        let (homepage,): (String,) = proxy.method_call("voxels.applications", "homepage", (uuid.to_string(),)).await?;
        let (_app_type,): (String,) = proxy.method_call("voxels.applications", "type", (uuid.to_string(),)).await?;

        Ok(
            Application::new(
                ApplicationRDN::new(rdn.as_str()).unwrap(),
                uuid,
                Some(Url::parse(&homepage).unwrap()),
                Some(description),
                None
            )
        )
    }

    pub fn rdn(&self) -> &ApplicationRDN {
        &self.rdn
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn homepage(&self) -> Option<&url::Url> {
        self.homepage.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn app_type(&self) -> Option<&ApplicationsType> {
        self.app_type.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_application() {
        let app_rdn =  ApplicationRDN::new("com.test").unwrap();
        let uuid = Uuid::new_v4();

        let app = Application::new(
           app_rdn.clone(),
           uuid.clone(),
           None,
           None,
           None,
        );

        assert_eq!(app.rdn(), &app_rdn);
        assert_eq!(app.id(), uuid);
    }
}
