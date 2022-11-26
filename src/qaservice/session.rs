use std::time::{Duration, Instant};
use log::{error, info, warn};
use crate::qaenv::localenv::CONFIG;
use actix::prelude::*;
use actix_web_actors::ws;

use crate::qaservice::tifiserver;
use crate::qaservice::tifi;
use serde_json::value::Value;
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsTradeSession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// joined topic
    pub topic: String,

    /// peer name
    pub name: Option<String>,

    /// Chat server
    pub addr: Addr<tifiserver::TifiServer>,
}

impl WsTradeSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(tifiserver::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsTradeSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with TradeServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(tifiserver::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(tifiserver::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<tifiserver::Message> for WsTradeSession {
    type Result = ();

    fn handle(&mut self, msg: tifiserver::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsTradeSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text)=> {

                let request: Value = match serde_json::from_str(&text.to_string().as_str()) {
                    Ok(x) => x,
                    Err(e) => {
                        error!("{:?}", e.to_string());
                        return ctx.text(e.to_string());
                    }
                };
                if let Some(topic) = request["aid"].as_str() {
                    match topic {
                        //{"topic":"aid","key":"000000000"}
                        "login" => {
                            let tifilogin = tifi::login{
                                aid: request["aid"].as_str().unwrap().to_string(),
                                bid: request["bid"].as_str().unwrap().to_string(),
                                user_name:request["user_name"].as_str().unwrap().to_string(),
                                password: request["password"].as_str().unwrap().to_string()
                            };
                            self.addr
                                .send(tifilogin)
                                .into_actor(self)
                        }
                        
                        _ => {}
                    }
                }
            }
           

            // ws::Message::Text(text) => {
            //     let m = text.trim();
            //     // we check for /sss type of messages
            //     if m.starts_with('/') {
            //         let v: Vec<&str> = m.splitn(2, ' ').collect();
            //         match v[0] {
            //             "/list" => {
            //                 // Send ListRooms message to chat server and wait for
            //                 // response
            //                 println!("List topics");
            //                 self.addr
            //                     .send(handler::ListTopics)
            //                     .into_actor(self)
            //                     .then(|res, _, ctx| {
            //                         match res {
            //                             Ok(topics) => {
            //                                 for topic in topics {
            //                                     ctx.text(topic);
            //                                 }
            //                             }
            //                             _ => println!("Something is wrong"),
            //                         }
            //                         fut::ready(())
            //                     })
            //                     .wait(ctx)
            //                 // .wait(ctx) pauses all events in context,
            //                 // so actor wont receive any new messages until it get list
            //                 // of rooms back
            //             }
            //             "/join" => {
            //                 if v.len() == 2 {
            //                     self.topic = v[1].to_owned();
            //                     self.addr.do_send(handler::Join {
            //                         id: self.id,
            //                         name: self.topic.clone(),
            //                     });

            //                     ctx.text("joined");
            //                 } else {
            //                     ctx.text("!!! topic name is required");
            //                 }
            //             }
            //             "/name" => {
            //                 if v.len() == 2 {
            //                     self.name = Some(v[1].to_owned());
            //                 } else {
            //                     ctx.text("!!! name is required");
            //                 }
            //             }
            //             "/list_account" => {
            //                 if v.len() == 2 {
            //                     self.addr.do_send(handler::ListAccounts {
                                    
            //                         topic: v[1].to_owned(),
            //                     });
            //                 } else {
            //                     ctx.text("!!! trade is required");
            //                 }
            //             }
            //             _ => ctx.text(format!("!!! unknown command: {m:?}")),
            //         }
            //     } else {
            //         let msg = if let Some(ref name) = self.name {
            //             format!("{name}: {m}")
            //         } else {
            //             m.to_owned()
            //         };
            //         // send message to chat server
            //         self.addr.do_send(handler::ClientMessage {
            //             id: self.id,
            //             msg,
            //             topic: self.topic.clone(),
            //         })
            //     }

                
            // }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}