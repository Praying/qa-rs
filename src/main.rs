// mod qaservice;
// use actix::*;
// use actix_files::{Files, NamedFile};
// use actix_web::{
//     middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
// };
// use actix_web_actors::ws;
// use qaservice::handler;
// use qaservice::session;
// use qaservice::tradeserver;
// use std::{
//     sync::{
//         atomic::{AtomicUsize, Ordering},
//         Arc,
//     },
//     time::Instant,
// };
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

//     // set up applications state
//     // keep a count of the number of visitors
//     let app_state = Arc::new(AtomicUsize::new(0));

//     // start chat server actor
//     let server = handler::TradeServer::new(app_state.clone()).start();

//     log::info!("starting HTTP server at http://localhost:8080");

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::from(app_state.clone()))
//             .app_data(web::Data::new(server.clone()))
//             .service(web::resource("/").to(tradeserver::index))
//             .route("/count", web::get().to(tradeserver::get_count))
//             .route("/ws", web::get().to(tradeserver::trade_route))
//             .service(Files::new("/static", "./static"))
//             .wrap(Logger::default())
//     })
//     .workers(6)
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

fn main() {}
