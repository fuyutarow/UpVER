pub use semver::Version;
use serde_derive::{Deserialize, Serialize};

pub trait Semver {
    fn set(
        &mut self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self;
    fn update(&mut self, major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self;
}

impl Semver for Version {
    fn set(
        &mut self,
        major: Option<u64>,
        minor: Option<u64>,
        patch: Option<u64>,
        pre: Option<String>,
        build: Option<String>,
    ) -> Self {
        Self {
            major: if let Some(n) = patch { n } else { self.major },
            minor: if let Some(n) = patch { n } else { self.minor },
            patch: if let Some(n) = patch { n } else { self.patch },
            pre: if let Some(s) = pre {
                vec![semver::Identifier::AlphaNumeric(s.into())]
            } else {
                self.pre.clone()
            },
            build: if let Some(s) = build {
                vec![semver::Identifier::AlphaNumeric(s.into())]
            } else {
                self.build.clone()
            },
        }
    }

    fn update(&mut self, major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self {
        Self {
            major: self.major + if let Some(n) = major { n } else { 0 },
            minor: self.minor + if let Some(n) = minor { n } else { 0 },
            patch: self.patch + if let Some(n) = patch { n } else { 0 },
            ..self.to_owned()
        }
    }
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    version: String,
}
