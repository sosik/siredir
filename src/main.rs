#![feature(trait_alias)]

use env_logger;
use futures::future::Future;
use hcontext;
use hyper;
use regex;
use yaps_hyper_router as router;
use std::net::SocketAddr;

use log;

mod config;
mod redirector;

const CMD_ARG_CONFIG: &str = "config";

// loads config from provided file or gets default config
fn load_config(path: Option<&str>) -> Result<config::Config, Box<std::error::Error>> {
    if let Some(p) = path {
        // path is defined so we will stritly load it
        config::Config::load(p).map_err(|e| {
            log::error!("Failed to load config file '{}'! {}", &p, e);
            e
        })
    } else {
        // path is not defined as cmd parameter, lets try defaut config file and fallback to default values

        config::Config::load("./siredir-config.yaml").or_else(|_| {
            log::warn!(
                "Failed to load default config file. Falling back to default configuration."
            );

            Ok(config::Config::new())
        })
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    log::info!("starting...");

    log::trace!("parsing command line parameters");
    let matches = clap::App::new("Siredir")
        .arg(
            clap::Arg::with_name(CMD_ARG_CONFIG)
                .short("c")
                .long("config")
                .takes_value(true),
        )
        .get_matches();

    let config = std::sync::Arc::new(load_config(matches.value_of(CMD_ARG_CONFIG))?);

    // configure context
    let ctx = std::sync::Arc::new(hcontext::HContext::new().with("config", config.clone()));

    // configure router
    let mut router = router::Router::new().with_context(ctx);

    let mut redirector = redirector::Redirector::new();
    for v in config.get_redirects().iter() {
        redirector.add_redirection(&v.re, v.rewrite_rule.to_string(), v.status_code)?;
    }

    router.add_route(regex::Regex::new(".*")?, redirector);

    // start server
    let bind_address = config
        .get_bind_to()
        .first()
        .expect("there has to be at least one bind_to address!")
        .to_owned();

    let server = hyper::Server::bind(
        &bind_address
            .parse::<SocketAddr>()
            .expect("Cannot parse bind_addr"),
    )
    .tcp_nodelay(true);

    log::trace!("Server bound to {}", &bind_address);

    let x = server.serve(move || {
        let r = router.clone();

        futures::future::ok::<router::Router, Box<std::error::Error + Send + Sync + 'static>>(r)
    });
    tokio::run(x.map_err(|e| eprintln!("{}", e)));

    log::info!("the end...");

    Ok(())
}
