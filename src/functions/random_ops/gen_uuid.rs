use uuid::{self, Timestamp, Uuid};

use crate::types::field::Field;

pub enum UUIDVersions {
    V1,
    V3,
    V4,
    V5,
}

struct GenUUID {
    version: UUIDVersions,
}

impl GenUUID {
    pub fn new(version: String) -> GenUUID {
        let version = match version.as_str() {
            "v1" => UUIDVersions::V1,
            "v3" => UUIDVersions::V3,
            "v4" => UUIDVersions::V4,
            "v5" => UUIDVersions::V5,
            _ => UUIDVersions::V4,
        };

        return GenUUID { version };
    }
}

impl Field for GenUUID {
    type Output = String;

    fn get_value(&self) -> Self::Output {
        return match self.version {
            UUIDVersions::V1 => todo!(),
            UUIDVersions::V3 => todo!(),
            UUIDVersions::V4 => Uuid::new_v4().to_string(),
            UUIDVersions::V5 => todo!(),
        };
    }
}
