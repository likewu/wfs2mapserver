#![allow(clippy::map_entry)]
use std::{collections::HashMap};
use std::pin::Pin;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use tracing::{error, info};
use http::{StatusCode, HeaderValue, request::{Request, Builder}, Response};
use url::{form_urlencoded, Url};
use tokio::sync::mpsc::{self, Receiver, Sender};

use common::error::{Error, Result};
use crate::bucket::lifecycle::bucket_cache::BucketLocationCache;

pub struct TransitionClient11 {
    pub secure: bool,
    pub s3_accelerate_endpoint: Arc<Mutex<String>>,
    pub region: String,
    pub endpoint_url: Url,
    pub bucket_loc_cache: Arc<Mutex<BucketLocationCache>>,
}

//pub struct TransitionCore11(pub Box<TransitionClient11>);
pub struct TransitionCore11(pub Arc<TransitionClient11>);

impl TransitionClient11 {
    pub async fn remove_objects_with_result(self: Arc<Self>, bucket_name: &str, objects_rx: Receiver<String>) -> Receiver<String> {
        let (result_tx, mut result_rx) = mpsc::channel(1);

        let self_clone = Arc::clone(&self);
        let bucket_name_owned = bucket_name.to_string();

        tokio::spawn(async move {
            self_clone.remove_objects_inner(&bucket_name_owned, objects_rx, &result_tx).await;
        });
        result_rx
    }

    pub async fn remove_objects_inner(&self, bucket_name: &str, mut objects_rx: Receiver<String>, result_tx: &Sender<String>) -> Result<()> {
        let max_entries = 1000;
        let mut finish = false;
        let mut url_values = HashMap::new();
        url_values.insert("delete".to_string(), "".to_string());

        let mut s3_accelerate_endpoint = self.s3_accelerate_endpoint.lock().await;
        self.set_s3_transfer_accelerate("aaaaa", s3_accelerate_endpoint.as_mut());

        let s3_accelerate_endpoint = self.s3_accelerate_endpoint.lock().await;
        self.set_s3_transfer_accelerate2("aaaaa", s3_accelerate_endpoint);

        let mut bucket_loc_cache = self.bucket_loc_cache.lock().await;
        self.get_bucket_location("bbb", &mut bucket_loc_cache).await;

        Ok(())
    }

    fn set_s3_transfer_accelerate(&self, accelerate_endpoint: &str, s3_accelerate_endpoint: &mut str) {
        let mut s3_accelerate_endpoint = String::from(s3_accelerate_endpoint);
        s3_accelerate_endpoint.push_str(accelerate_endpoint);
    }

    fn set_s3_transfer_accelerate2(&self, accelerate_endpoint: &str, mut s3_accelerate_endpoint: MutexGuard<String>) {
        *s3_accelerate_endpoint = accelerate_endpoint.to_string();
    }

    pub async fn get_bucket_location(&self, bucket_name: &str, bucket_loc_cache: &mut BucketLocationCache) -> Result<String> {
        Ok(self.get_bucket_location_inner(bucket_name, bucket_loc_cache).await?)
    }

    async fn get_bucket_location_inner(&self, bucket_name: &str, bucket_loc_cache: &mut BucketLocationCache) -> Result<String> {
        let location = bucket_loc_cache.get(bucket_name);
        if let Ok(location) = location {
            return Ok(location);
        }

        let location = "aaa".to_string();
        bucket_loc_cache.set(bucket_name, &location);
        Ok(location)
    }
}

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
            secure: false,
            region: tier.to_string(),
            s3_accelerate_endpoint: Arc::new(Mutex::new("".to_string())),
            endpoint_url: Url::parse(s1).unwrap(),
            bucket_loc_cache: Arc::new(Mutex::new(BucketLocationCache::new())),
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
        secure: false,
        region: tier.to_string(),
        s3_accelerate_endpoint: Arc::new(Mutex::new("".to_string())),
        endpoint_url: Url::parse(s1).unwrap(),
        bucket_loc_cache: Arc::new(Mutex::new(BucketLocationCache::new())),
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

use smart_default::SmartDefault;

#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
        r#type: String,
    },
    Qux(i32),
}