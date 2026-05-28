use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[derive(Debug)]
pub enum Visibility {
    Secret,
    Private,
    Public,
}

impl Visibility {
    pub fn can_be_viewed_by_owner(&self) -> bool {
        match self {
            Self::Secret => false,
            Self::Public => true,
            Self::Private => true,
        }
    }
    pub fn can_be_viewed_by_not_owner(&self) -> bool {
        match self {
            Self::Secret => false,
            Self::Public => true,
            Self::Private => false,
        }
    }
}
