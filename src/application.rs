use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationErrors {
    InvalidName,
}

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub enum ApplicationsType {
    Client,
    Server,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationRDN {
    name: String,
}

impl ApplicationRDN {
    pub fn new(name: String) -> Result<ApplicationRDN, ApplicationErrors> {
        if name.len() > 255 || name.is_empty() {
            return Err(ApplicationErrors::InvalidName);
        }

        if name.chars().filter(|&c| c == '.').count() == 0 {
            return Err(ApplicationErrors::InvalidName);
        }

        for element in name.split('.') {
            if element.is_empty() {
                return Err(ApplicationErrors::InvalidName);
            }

            if element.chars().nth(0).unwrap().is_numeric() {
                return Err(ApplicationErrors::InvalidName);
            }
        }

        if name.chars().filter(|&c| !c.is_alphanumeric() && c != '_' && c != '.').count().gt(&0) {
            return Err(ApplicationErrors::InvalidName);
        }

        Ok(ApplicationRDN {
            name
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    rdn: ApplicationRDN,
    id: uuid::Uuid,
    homepage: Option<url::Url>,
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