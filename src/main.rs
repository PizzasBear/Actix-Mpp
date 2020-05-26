// #[macro_use]
// extern crate serde_json;

// #[macro_use]
// extern crate actix_web;

mod config;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
// use std::sync::Mutex;
use pg::{Client, NoTls};
use tokio_postgres as pg;

use config::Config;
use handlebars::Handlebars;
// use serde_json::json;

use std::io;

macro_rules! ron {
    ($($arg:tt)*) => {{
        ron::de::from_str::<ron::Value>(&format!($($arg)*)[..])
    }}
}

macro_rules! other_err {
    ($e:expr) => {
        Err(io::Error::new(io::ErrorKind::Other, $e))
    };
}

struct AppState<'a> {
    pub hb: Handlebars<'a>,
    pub pg_client: Client,
}

async fn index(req: HttpRequest, data: web::Data<AppState<'_>>) -> io::Result<impl Responder> {
    let info = req.connection_info();

    let mut counter: i32 = match data.pg_client.query("SELECT idx FROM data", &[]).await {
        Ok(o) => o,
        Err(e) => return other_err!(e),
    }[0]
    .get(0);

    let hb_data = match ron!(r#"(count: {}, address: "{}")"#, counter, info.host()) {
        Ok(o) => o,
        Err(e) => return other_err!(e),
    };

    counter += 1;

    if let Err(e) = data
        .pg_client
        .execute("UPDATE data SET idx = $1", &[&counter])
        .await
    {
        return other_err!(e);
    }

    Ok(HttpResponse::Ok().body(data.hb.render("index", &hb_data).unwrap()))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // -- config --
    let cfg = Config::read("./Conf.ron")?;

    // -- postgres --
    let (client, connection) = match pg::connect(&cfg.pg_config, NoTls).await {
        Ok(a) => a,
        Err(e) => return other_err!(e),
    };

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // -- state --
    let mut state = AppState {
        hb: Handlebars::new(),
        pg_client: client,
    };

    if let Err(e) = state.hb.register_templates_directory(".hbs", "./templates") {
        return other_err!(e);
    }

    // -- http server --
    let state_ref = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state_ref.clone())
            .service(web::resource("/").to(index))
    });

    server.bind(cfg.address)?.run().await
}
