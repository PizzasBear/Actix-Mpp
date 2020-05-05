// #[macro_use]
// extern crate serde_json;

// #[macro_use]
// extern crate actix_web;

mod config;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use tokio_postgres::{NoTls, Client};

use handlebars::Handlebars;
use config::{Config};
use serde_json::json;

struct AppState<'a> {
    pub hb: Handlebars<'a>,
    pub pg_client: Mutex<Client>,
}

async fn index(req: HttpRequest, data: web::Data<AppState<'_>>) -> impl Responder {
    let client = data.pg_client.lock().unwrap();
    let info = req.connection_info();

    let mut counter: i32 = client
        .query("SELECT idx FROM data", &[])
        .await.unwrap()[0].get(0);

    let json_data = json!({
        "count": counter,
        "address": info.host()
    });

    counter += 1;

    client.execute("UPDATE data SET idx = $1", &[&counter]).await.unwrap();
    
    HttpResponse::Ok().body(data.hb.render("index", &json_data).unwrap())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // -- config --
    let cfg = Config::read(&"./Conf.json");

    // -- postgres --
    let (client, connection) =
        match tokio_postgres::connect(&cfg.pg_config, NoTls).await {
            Ok(a) => a,
            Err(e) => panic!(format!("Couldn't connect to postgres {}", e)),
        };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // -- state --
    let mut state = AppState {
        hb: Handlebars::new(),
        pg_client: Mutex::new(client),
    };
    state.hb
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    // -- http server --
    let state_ref = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state_ref.clone())
            .service(web::resource("/").to(index))
            // .route("/", web::get().to(index))
    });
    
    if cfg.is_unix_address {
        server.bind_uds(cfg.address)?
        .run()
        .await
    }
    else {
        server.bind(cfg.address)?
        .run()
        .await
    }
}
