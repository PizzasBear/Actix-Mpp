// #[macro_use]
// extern crate serde_json;

// #[macro_use]
// extern crate actix_web;

mod config;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
// use std::sync::Mutex;
use pg::{Client, NoTls};
use tokio_postgres as pg;

use config::Config;
use handlebars::Handlebars;
// use serde_json::json;

use std::error::Error;

#[derive(Debug)]
struct GeneralError(Box<dyn Error>);

impl<T: Error + 'static> From<T> for GeneralError {
    fn from(err: T) -> Self {
        let boxed_err = Box::new(err) as Box<dyn Error>;
        Self(boxed_err)
    }
}

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeneralError( {} )", self.0)
    }
}

impl actix_web::ResponseError for GeneralError {}

macro_rules! ron {
    ($($arg:tt)*) => {{
        ron::de::from_str::<ron::Value>(&format!($($arg)*)[..])
    }}
}

struct AppState<'a> {
    pub hb: Handlebars<'a>,
    pub pg_client: Client,
}

async fn index(
    req: HttpRequest,
    data: web::Data<AppState<'_>>,
) -> Result<HttpResponse, GeneralError> {
    let info = req.connection_info();

    let mut counter: i32 = data.pg_client.query("SELECT idx FROM data", &[]).await?[0].get(0);

    let hb_data = ron!(r#"(count: {}, address: "{}")"#, counter, info.host())?;

    counter += 1;

    data.pg_client
        .execute("UPDATE data SET idx = $1", &[&counter])
        .await?;

    Ok(HttpResponse::Ok().body(data.hb.render("index", &hb_data)?))
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // -- config --
    let cfg = Config::read("./Conf.ron")?;

    // -- postgres --
    let (client, connection) = pg::connect(&cfg.pg_config, NoTls).await?;

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

    state
        .hb
        .register_templates_directory(".hbs", "./templates")?;

    // -- http server --
    let state_ref = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state_ref.clone())
            .service(web::resource("/").to(index))
    });

    server.bind(cfg.address)?.run().await?;

    Ok(())
}
