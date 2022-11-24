use std::time::{Duration, SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use crate::types::Field;

pub enum UUIDVersions {
    V1,
    V3,
    V4,
    V5,
}

pub struct GenUUID {
    version: UUIDVersions,
    namespace: Uuid,
    name: [u8; 16],
    node: [u8; 6],
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

        let duration: Duration = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(v) => v,
            Err(_) => Duration::new(0, 0),
        };
        let duration_bytes = duration.as_nanos().to_be_bytes();

        return GenUUID {
            version,
            namespace: Uuid::NAMESPACE_X500,
            name: Self::get_limited_bytes::<16>(&duration_bytes),
            node: Self::get_limited_bytes::<6>(&duration_bytes),
        };
    }

    fn get_limited_bytes<const LIMIT: usize>(bytes: &[u8]) -> [u8; LIMIT] {
        let size: usize;
        if bytes.len() > LIMIT {
            size = LIMIT;
        } else {
            size = bytes.len();
        }

        let mut node: [u8; LIMIT] = [0; LIMIT];
        for i in 0..size {
            node[i] = bytes[bytes.len() - i - 1];
        }

        return node;
    }
}

impl Field for GenUUID {
    type Output = String;

    fn get_value(&self) -> Self::Output {
        return match self.version {
            UUIDVersions::V1 => Uuid::now_v1(&self.node).to_string(),
            UUIDVersions::V3 => Uuid::new_v3(&self.namespace, &self.name).to_string(),
            UUIDVersions::V4 => Uuid::new_v4().to_string(),
            UUIDVersions::V5 => Uuid::new_v5(&self.namespace, &self.name).to_string(),
        };
    }
}
