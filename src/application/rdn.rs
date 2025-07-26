use std::path::{Path, PathBuf};
#[cfg(feature = "dbus")]
use dbus::arg::{Iter, IterAppend, TypeMismatchError};
use dbus::arg::ArgType;
use dbus::Signature;
use serde::{Deserialize, Serialize};

use thiserror::Error;
use crate::application::application::Application;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum ApplicationRDNErrors {
    #[error("An application RDN should be less than 255 characters in length")]
    TooLong,
    #[error("An application RDN should not be an empty string ")]
    Empty,
    #[error("An application RDN must have at least one separating character: '.'")]
    MissingSeparators,
    #[error("An application RDN must not have two separating characters adjacent together e.g: 'test..com'")]
    DoubleSeparator,
    #[error("An applications RDN must not have a segment that begins with a numeric character e.g: 'test.9hello.com")]
    StartsWithNumeric,
    #[error("An application RDN must not have segments composed of anything other than alphanumerics and underscores")]
    InvalidCharacter
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationRDN {
    name: String,
}

#[cfg(feature = "dbus")]
impl dbus::arg::Append for ApplicationRDN {
    fn append(self, iter: &mut IterAppend) where Self: Sized {
        iter.append_struct(|s| {
            s.append(&self.name);
        });
    }

    fn append_by_ref(&self, iter: &mut IterAppend) {
        iter.append_struct(|s| {
            s.append(&self.name);
        });
    }
}

#[cfg(feature = "dbus")]
impl dbus::arg::Get<'_> for ApplicationRDN {
    fn get(iter: &mut Iter) -> Option<Self> {
        let name: String = iter.get()?;

        let result = ApplicationRDN::new(name.as_str());

        if result.is_ok() {
            Some(result.unwrap())
        } else {
            None
        }
    }
}


#[cfg(feature = "dbus")]
impl dbus::arg::Arg for ApplicationRDN {
    const ARG_TYPE: ArgType = ArgType::Struct;

    fn signature() -> Signature<'static> {
        Signature::make::<Self>()
    }
}


impl ApplicationRDN {
    pub fn new(name: &str) -> Result<ApplicationRDN, ApplicationRDNErrors> {
        if name.len() > 255 {
            return Err(ApplicationRDNErrors::TooLong);
        }

        if name.is_empty() {
            return Err(ApplicationRDNErrors::Empty);
        }

        if name.chars().filter(|&c| c == '.').count() == 0 {
            return Err(ApplicationRDNErrors::MissingSeparators);
        }

        for element in name.split('.') {
            if element.is_empty() {
                return Err(ApplicationRDNErrors::DoubleSeparator);
            }

            if element.chars().nth(0).unwrap().is_numeric() {
                return Err(ApplicationRDNErrors::StartsWithNumeric);
            }
        }

        if name.chars().filter(|&c| !c.is_alphanumeric() && c != '_' && c != '.').count().gt(&0) {
            return Err(ApplicationRDNErrors::InvalidCharacter);
        }

        Ok(ApplicationRDN {
            name: String::from(name)
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn as_path(&self) -> PathBuf {
        let mut result: PathBuf = PathBuf::new();

        for element in self.name.split('.') {
            result = result.join(element).to_owned();
        }

        result
    }

    pub fn as_path_with_prefix(&self, prefix: &Path) -> PathBuf {
        let result: PathBuf = self.as_path();

        prefix.to_owned().join(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn as_path() {
        let test_rdn = ApplicationRDN::new("test.com").unwrap();

        assert_eq!(
            test_rdn.as_path(),
            PathBuf::from("test/com")
        )
    }

    #[test]
    fn as_path_with_prefix() {
        let test_rdn = ApplicationRDN::new("com.test").unwrap();

        assert_eq!(
            test_rdn.as_path_with_prefix(Path::new("/prefix/to/")),
            PathBuf::from("/prefix/to/com/test")
        );

        assert_eq!(
            test_rdn.as_path_with_prefix(Path::new("/")),
            PathBuf::from("/com/test")
        )
    }
}