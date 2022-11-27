extern crate mongodb;

use std::result;

use crate::qaaccount::account::QA_Account;
use crate::qaenv::localenv::CONFIG;
use crate::qaprotocol::qifi::account::QIFI;
use crate::qaprotocol::qifi::func::{from_serde_value, from_string};
use chrono::Local;
use futures::TryStreamExt;
use lazy_static::lazy_static;
pub use mongodb::{
    bson::{doc, Bson, Document},
    error::Result,
    options::{
        DeleteOptions, FindOneOptions, FindOptions, InsertOneOptions, UpdateModifications,
        UpdateOptions,
    },
    sync::Client,
    Collection, IndexModel,
};

use async_once::AsyncOnce;
use serde::Serialize;
pub fn struct_to_doc<T>(value: T) -> Document
where
    T: Serialize + std::fmt::Debug,
{
    mongodb::bson::to_bson(&value)
        .unwrap()
        .as_document()
        .unwrap()
        .to_owned()
}
lazy_static! {
    //pub static ref MARKET_MONGO: Client = create_market_mongo_client();
    pub static ref CLIENT:QAMongoClient = QAMongoClient::new(&CONFIG.account.uri);
}
#[derive(Debug, Clone)]
pub struct QAMongoClient {
    pub client: Client,
}

impl QAMongoClient {
    pub fn new(uri: &str) -> Self {
        let client = Client::with_uri_str(uri).unwrap();

        Self { client }
    }
    pub fn check_user(&self, account_cookie: &str, password: &str) -> bool {
        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("account");
        let doc = doc! {"account_cookie": account_cookie, "password": password};
        let cursor = coll.count_documents(doc, None).unwrap();
        if cursor > 0 {
            true
        } else {
            false
        }
    }
    pub fn get_qifi(&self, account_cookie: String) -> QIFI {
        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("account");
        let cursor = coll
            .find_one(doc! {"account_cookie": &account_cookie}, None)
            .expect("Failed to execute find.");
        cursor.unwrap()
    }
    pub fn get_account(&self, account_cookie: String) -> QA_Account {
        // 转换为结构体
        let c: QIFI = self.get_qifi(account_cookie);
        // println!("{:#?}", c);
        QA_Account::new_from_qifi(c)
    }

    pub fn get_accountlist(&self) -> Vec<String> {
        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("account");
        let mut cursor = coll
            .find(
                None,
                FindOptions::builder()
                    .batch_size(Option::from(10000000))
                    .projection(Option::from(doc! {"account_cookie": 1}))
                    .build(),
            )
            .expect("Failed to execute find.");

        let mut u: Vec<String> = Vec::new();
        for result in cursor {
            u.push(result.unwrap().account_cookie.to_string());
        }
        u
    }

    pub fn save_account(&self, mut account: QA_Account) {
        // 实时切片
        let slice: QIFI = account.get_qifi_slice();
        self.save_qifi_slice(slice);
    }

    pub fn save_qifi_slice(&self, mut slice: QIFI) {
        slice.updatetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("account");

        let v = struct_to_doc(slice.clone());
        coll.update_one(
            doc! {"account_cookie": slice.account_cookie},
            v,
            UpdateOptions::builder().upsert(Option::from(true)).build(),
        )
        .unwrap();
    }

    pub fn save_his_qifi_slice(&self, slice: QIFI) {
        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("history");
        let trading_day = slice.trading_day.clone();
        let v = struct_to_doc(slice.clone());
        coll.update_one(
            doc! {"account_cookie": slice.account_cookie,"trading_day":trading_day},
            v,
            UpdateOptions::builder().upsert(Option::from(true)).build(),
        )
        .unwrap();
    }

    pub fn save_accounthis(&self, mut account: QA_Account) {
        let slice: QIFI = account.get_qifi_slice();

        let coll = self
            .client
            .database(CONFIG.account.db.as_str())
            .collection::<QIFI>("history");
        let trading_day = slice.trading_day.clone();
        let v = struct_to_doc(slice.clone());
        coll.update_one(
            doc! {"account_cookie": slice.account_cookie,"trading_day":trading_day},
            v,
            UpdateOptions::builder().upsert(Option::from(true)).build(),
        )
        .unwrap();
    }
}
