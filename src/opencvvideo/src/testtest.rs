#![allow(clippy::map_entry)]
use std::pin::Pin;
use std::fmt::Display;
use std::sync::Arc;
use tracing::warn;
use tracing::{error, info};

use crate::error::{Error, Result};

pub struct TransitionClient11 {
    pub endpoint_url: String,
    pub secure: bool,
    pub region: String,
}

//pub struct TransitionCore11(pub Box<TransitionClient11>);
pub struct TransitionCore11(pub Arc<TransitionClient11>);

pub struct WarmBackendS3 {
    //pub core: TransitionCore11,
    //pub client: Box<TransitionClient11>,
    pub core11: TransitionCore11,
    pub client11: Arc<TransitionClient11>,
    pub bucket: String,
    pub prefix: String,
    pub storage_class: String,
}

impl WarmBackendS3 {
    /*pub fn new(s1: &str, tier: &str) -> Result<Self> {  //Box can not clone
        let client = TransitionClient11 {
            endpoint_url: s1.to_string(),
            secure: false,
            region: tier.to_string(),
        };
        let client = Box::new(client);
        let core = TransitionCore11(client);

        Ok(Self {
            core,
            client,
            bucket:        "".to_string(),
            prefix:        "".to_string(),
            storage_class: "".to_string(),
        })
    }*/

    pub fn new11(s1: &str, tier: &str) -> Result<Self> {
        let client = TransitionClient11 {
            endpoint_url: s1.to_string(),
            secure: false,
            region: tier.to_string(),
        };
        let client11 = Arc::new(client);
        let client111 = Arc::clone(&client11);
        let core11 = TransitionCore11(client11);

        Ok(Self {
            core11,
            client11: client111,
            bucket:        "".to_string(),
            prefix:        "".to_string(),
            storage_class: "".to_string(),
        })
    }
}

fn test_move_box(s1: &str, tier: &str) -> Box<TransitionClient11> {
    let client = TransitionClient11 {
        endpoint_url: s1.to_string(),
        secure: false,
        region: tier.to_string(),
    };
    let client = Box::new(client);
    client
}

/*
fn test_move_ref<'a>(s1: &'a str, tier: &'a str) -> &'a TransitionClient11 {  //cannot return reference to local variable
    let client = TransitionClient11 {
        endpoint_url: s1.to_string(),
        secure: false,
        region: tier.to_string(),
    };
    &client
}
*/