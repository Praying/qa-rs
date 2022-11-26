use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use crate::qaservice::tifiserver;
use crate::qaservice::session;
use actix::*;
use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Entry point for our websocket route
pub async fn trade_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<tifiserver::TifiServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsTradeSession {
            id: 0,
            hb: Instant::now(),
            topic: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
pub async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}
