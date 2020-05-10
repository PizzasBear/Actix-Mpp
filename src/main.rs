// #[macro_use]
// extern crate serde_json;

// #[macro_use]
// extern crate actix_web;

mod config;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use tokio_postgres::{Client, NoTls};

use config::Config;
use handlebars::Handlebars;
// use serde_json::json;

use std::io;

struct AppState<'a> {
    pub hb: Handlebars<'a>,
    pub pg_client: Mutex<Client>,
}

macro_rules! ron {
    ($($arg:tt)*) => {{
        ron::de::from_str::<ron::Value>(&format!($($arg)*)[..])
    }}
}

async fn index(req: HttpRequest, data: web::Data<AppState<'_>>) -> io::Result<impl Responder> {
    let client = data.pg_client.lock().unwrap();
    let info = req.connection_info();

    let mut counter: i32 = client.query("SELECT idx FROM data", &[]).await.unwrap()[0].get(0);

    let hb_data = match ron!(r#"(count: {}, address: "{}")"#, counter, info.host()) {
        Ok(o) => o,
        Err(e) => { println!("ron Error"); return Err(io::Error::new(io::ErrorKind::InvalidData, e)) },
    };

    counter += 1;

    client
        .execute("UPDATE data SET idx = $1", &[&counter])
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(data.hb.render("index", &hb_data).unwrap()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // -- config --
    let cfg = Config::read("./Conf.ron")?;

    // -- postgres --
    let (client, connection) = match tokio_postgres::connect(&cfg.pg_config, NoTls).await {
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
    state
        .hb
        .register_templates_directory(".hbs", "./templates")
        .unwrap();

    // -- http server --
    let state_ref = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state_ref.clone())
            .service(web::resource("/").to(index))
            .service(web::resource("//").to(|| {
                HttpResponse::PermanentRedirect()
                    .header("Location", "/")
                    .finish()
            }))
    });

    server.bind(cfg.address)?.run().await
}
