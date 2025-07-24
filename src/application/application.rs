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
    pub fn new(rdn: ApplicationRDN, id: uuid::Uuid, homepage: Option<url::Url>, description: Option<String>, app_type: Option<ApplicationsType>) -> Application {
        Application {
            rdn,
            id,
            homepage,
            description,
            app_type
        }
    }
    
    pub fn using_dbus() -> Application {
            
        todo!()
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
