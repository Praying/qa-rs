use crate::qaconnector::mongo::mongoclient::CLIENT;
use crate::qaservice::tifi;
use crate::qamarket::marketsys::QAMarket;


use actix::prelude::*;
use hashbrown::HashMap;
use rand::{self, rngs::ThreadRng, Rng};
use std::{
    collections::{ HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};




pub struct TifiServer {
    sessions: HashMap<usize, Recipient<Message>>,
    brokers: HashMap<String, HashSet<usize>>, // broker --> something like tradeid
    
    rng: ThreadRng,
    market: QAMarket,
    visitor_count: Arc<AtomicUsize>,
}

impl TifiServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> TifiServer {
        // default Broker
        let mut brokers = HashMap::new();
        brokers.insert("qaexchange".to_owned(), HashSet::new());

        TifiServer {
            sessions: HashMap::new(),
            brokers,
            rng: rand::thread_rng(),
            visitor_count,
            market: QAMarket::new_for_realtime("qaexchange"),
        }
    }
}

impl TifiServer {
    /// Send message to all users in the Broker
    ///
    /// tell everyone in sometoic about the price change
    /// every
    fn update_price(&self, Broker: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.brokers.get(Broker) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}
/// Make actor from `TradeServer`
impl Actor for TifiServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<tifi::login> for TifiServer {
    type Result = MessageResult<tifi::login>;

    fn handle(&mut self, msg: tifi::login, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");
        // login && try to reload the account from mongodb
        let account_cookie = msg.user_name;
        let password = msg.password;
        if CLIENT.check_user(account_cookie, password) {
            // notify all users in same Broker
            self.update_price("main", "Someone joined", 0);
            // register session with random id
            let id = self.rng.gen::<usize>();
            self.sessions.insert(id, msg.addr);
            MessageResult(tifi::login::new(id, true))
        } else {
            MessageResult(tifi::login::new(0, false))
        }
        ()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

impl Handler<Connect> for TifiServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same Broker
        self.update_price("main", "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main Broker
        self.brokers
            .entry("main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.update_price("main", &format!("Total visitors {count}"), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for TifiServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut brokers: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all brokers
            for (name, sessions) in &mut self.brokers {
                if sessions.remove(&msg.id) {
                    brokers.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for Broker in brokers {
            self.update_price(&Broker, "Someone disconnected", 0);
        }
    }
}
