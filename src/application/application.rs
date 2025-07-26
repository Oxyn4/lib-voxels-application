use std::marker::PhantomData;
use std::time::Duration;
use arg::Variant;
#[cfg(feature = "dbus")]
use dbus::arg::{IterAppend, TypeMismatchError};
use dbus::arg::{Append, ArgType, Iter};
use dbus::{arg, Signature};
#[cfg(feature = "dbus")]
use dbus_tokio::connection::IOResourceError;

use super::rdn::{ApplicationRDNErrors, ApplicationRDN};

use serde::{Serialize, Deserialize};

use thiserror::Error;

use uuid::Uuid;
use url::Url;

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_APPLICATIONS_GET_PATH: &str = "get";

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_APPLICATIONS_RDN_METHOD: &str = "rdn";

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_APPLICATIONS_HOMEPAGE_METHOD: &str = "homepage";

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_APPLICATIONS_DESCRIPTION_METHOD: &str = "description";

#[cfg(feature = "dbus")]
pub const DBUS_STANDARD_VOXELS_APPLICATIONS_TYPE_METHOD: &str = "type";

#[cfg(feature = "dbus")]
pub struct Empty {}

#[cfg(feature = "dbus")]
impl Append for Empty {
    fn append(self, iter: &mut IterAppend) where Self: Sized {
        iter.append(0);
    }

    fn append_by_ref(&self, iter: &mut IterAppend) {
        iter.append(0);
    }
}

#[cfg(feature = "dbus")]
impl arg::Get<'_> for Empty {
    fn get(iter: &mut Iter) -> Option<Self> {
        let _: u8 = iter.get()?;

        Some(Empty{})
    }
}

#[cfg(feature = "dbus")]
impl arg::Arg for Empty {
    const ARG_TYPE: ArgType = ArgType::Byte;

    fn signature() -> Signature<'static> {
        Signature::make::<Self>()
    }
}

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ApplicationErrors {
    #[error("Invalid RDN provided for application")]
    InvalidRDN(ApplicationRDNErrors),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationsType {
    Client,
    Server,
    Other(String),
}

#[cfg(feature = "dbus")]
impl Append for ApplicationsType {
    fn append(self, iter: &mut IterAppend) where Self: Sized {
        match self {
            Self::Client => (iter.append("Client")),
            Self::Server => iter.append("Server"),
            Self::Other(name) => iter.append(name),
        }
    }

    fn append_by_ref(&self, iter: &mut IterAppend) {
        match self {
            Self::Client => (iter.append("Client")),
            Self::Server => iter.append("Server"),
            Self::Other(name) => iter.append(name),
        }
    }
}

#[cfg(feature = "dbus")]
impl arg::Get<'_> for ApplicationsType {
    fn get(i: &mut Iter) -> Option<Self> {
        let read= i.read();

        if read.is_err() {
            return None;
        }

        let (tag,): (String,) = read.unwrap();

        Some(
            match tag.as_ref() {
                "Client" => ApplicationsType::Client,
                "Server" => ApplicationsType::Server,
                _ => ApplicationsType::Other(tag),
            }
        )
    }
}

#[cfg(feature = "dbus")]
impl arg::Arg for ApplicationsType {
    const ARG_TYPE: ArgType = ArgType::Variant;

    fn signature() -> Signature<'static> {
        Signature::make::<Self>()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    rdn: ApplicationRDN,
    id: Uuid,
    homepage: Option<Url>,
    description: Option<String>,
    app_type: Option<ApplicationsType>
}

#[cfg(feature = "dbus")]
impl Append for Application {
    fn append(self, iter: &mut IterAppend) where Self: Sized {
        iter.append_struct(|s| {
            s.append(&self.rdn);
            s.append(self.id.to_string());

            match &self.homepage {
                Some(u) => {
                    s.append(Variant(u.to_string()))
                }
                None => {
                    s.append(Variant(Empty{}))
                }
            }

            match &self.description {
                Some(d) => {
                    s.append(Variant(d.to_string()))
                }
                None => {
                    s.append(Variant(Empty{}))
                }
            }

            match &self.app_type {
                Some(a) => {
                    s.append(Variant(a))
                },
                None => {
                    s.append(Variant(Empty{}))
                }
            }
        });
    }

    fn append_by_ref(&self, iter: &mut IterAppend) {
        iter.append_struct(|s| {
            s.append(&self.rdn);
            s.append(self.id.to_string());

            match &self.homepage {
                Some(u) => {
                    s.append(Variant(u.to_string()))
                }
                None => {
                    s.append(Variant(Empty{}))
                }
            }

            match &self.description {
                Some(d) => {
                    s.append(Variant(d.to_string()))
                }
                None => {
                    s.append(Variant(Empty{}))
                }
            }

            match &self.app_type {
                Some(a) => {
                    s.append(Variant(a))
                },
                None => {
                    s.append(Variant(Empty{}))
                }
            }
        });
    }
}

#[cfg(feature = "dbus")]
impl arg::Arg for Application {
    const ARG_TYPE: ArgType = ArgType::Struct;

    fn signature() -> Signature<'static> {
        Signature::make::<Self>()
    }
}

#[cfg(feature = "dbus")]
impl arg::Get<'_> for Application {
    fn get(i: &mut Iter) -> Option<Self> {
        let read = i.read();

        if read.is_err() {
            return None;
        }

        let (rdn, uuid, homepage, description, app_type): (ApplicationRDN, String, String, String, ApplicationsType) = read.unwrap();

        todo!()
    }
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

        let proxy = dbus::nonblock::Proxy::new("voxels.applications", "/get", Duration::from_secs(2), con.clone());

        let (rdn,): (String,) = proxy.method_call("voxels.applications", "rdn", (uuid.to_string(),)).await?;
        let (description,): (String,) = proxy.method_call("voxels.applications", "description", (uuid.to_string(),)).await?;
        let (homepage,): (String,) = proxy.method_call("voxels.applications", "homepage", (uuid.to_string(),)).await?;
        let (_app_type,): (String,) = proxy.method_call("voxels.applications", "type", (uuid.to_string(),)).await?;

        drop(cancel_token);
        drop(con);

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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn homepage(&self) -> Option<&Url> {
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
