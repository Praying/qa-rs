use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

/// Chat server sends this messages to session
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

/// Send message to specific topic
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub topic: String,
}

/// List of available topics
pub struct ListTopics;

impl actix::Message for ListTopics {
    type Result = Vec<String>;
}

/// Join topic, if topic does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client ID
    pub id: usize,

    /// Room name
    pub name: String,
}

/// `TradeServer` manages chat topics and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct TradeServer {
    sessions: HashMap<usize, Recipient<Message>>,
    topics: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl TradeServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> TradeServer {
        // default topic
        let mut topics = HashMap::new();
        topics.insert("main".to_owned(), HashSet::new());

        TradeServer {
            sessions: HashMap::new(),
            topics,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl TradeServer {
    /// Send message to all users in the topic
    fn send_message(&self, topic: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.topics.get(topic) {
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
impl Actor for TradeServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for TradeServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same topic
        self.send_message("main", "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main topic
        self.topics
            .entry("main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message("main", &format!("Total visitors {count}"), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for TradeServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut topics: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all topics
            for (name, sessions) in &mut self.topics {
                if sessions.remove(&msg.id) {
                    topics.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for topic in topics {
            self.send_message(&topic, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for TradeServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.topic, msg.msg.as_str(), msg.id);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListTopics> for TradeServer {
    type Result = MessageResult<ListTopics>;

    fn handle(&mut self, _: ListTopics, _: &mut Context<Self>) -> Self::Result {
        let mut topics = Vec::new();

        for key in self.topics.keys() {
            topics.push(key.to_owned())
        }

        MessageResult(topics)
    }
}

/// Join topic, send disconnect message to old topic
/// send join message to new topic
impl Handler<Join> for TradeServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut topics = Vec::new();

        // remove session from all topics
        for (n, sessions) in &mut self.topics {
            if sessions.remove(&id) {
                topics.push(n.to_owned());
            }
        }
        // send message to other users
        for topic in topics {
            self.send_message(&topic, "Someone disconnected", 0);
        }

        self.topics
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}
