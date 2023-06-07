use std::collections::HashMap;
use actix_web::{middleware, web, App, HttpServer};


use super::database;
use super::http::handlers;



pub async fn server(ip: String, config_db: HashMap<String, String>) -> std::io::Result<()> {
    log::info!("starting HTTP server at http://{}:8082", &ip);

    let pool = database::create_pool(config_db);

    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // <- limit size of the payload (global configuration)
            .service(web::resource("/signIn").route(web::post().to(handlers::sign_in)))
            .service(web::resource("/signOut").route(web::post().to(handlers::sign_out)))
            .service(web::resource("/account").route(web::post().to(handlers::account)))
            .service(web::resource("/trades").route(web::post().to(handlers::trade)))
            .service(web::resource("/position").route(web::post().to(handlers::positions)))
            .service(web::resource("/net_worth").route(web::post().to(handlers::net_worth)))
            .service(web::resource("/income").route(web::post().to(handlers::incomes)))
            .service(web::resource("/equity").route(web::post().to(handlers::pnl)))
            .service(web::resource("/newPrice").route(web::post().to(handlers::is_price)))
            .service(web::resource("/incomes").route(web::post().to(handlers::history_incomes)))
            .service(web::resource("/open_orders").route(web::post().to(handlers::open_orders)))
            .service(web::resource("/accounts").route(web::post().to(handlers::assets)))
            .service(web::resource("/date_history").route(web::post().to(handlers::date_trade)))
            .service(web::resource("/products").route(web::post().to(handlers::get_products_data)))
            .service(web::resource("/alarm_orders").route(web::post().to(handlers::get_open_orders_data)))
            .service(web::resource("/delect_orders").route(web::post().to(handlers::delect_open_orders_data)))
            .service(web::resource("/add_orders").route(web::post().to(handlers::add_open_orders_data)))
            .service(web::resource("/get_positions").route(web::post().to(handlers::get_positions_data)))
            .service(web::resource("/delect_positions").route(web::post().to(handlers::delect_positions_data)))
            .service(web::resource("/add_positions").route(web::post().to(handlers::add_positions_data)))
            .service(web::resource("/update_positions").route(web::post().to(handlers::update_positions_data)))
            .service(web::resource("/update_ori_balance").route(web::post().to(handlers::update_ori_balance_data)))
            .service(web::resource("/get_accounts").route(web::post().to(handlers::get_account)))
            
    })
    .bind((ip.as_str(), 8082))?
    .run();

    return server.await;
}