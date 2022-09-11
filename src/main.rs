use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use botip::args::Args;
use env_logger::Env;
use log::{error, info};
use std::collections::HashMap;
use std::io::Result;

#[derive(Template)]
#[template(path = "index.html")]
struct TemplateContext {
    ip_address_v4: String,
}

fn request_is_bot(user_agent: &str) -> bool {
    if user_agent.contains("curl") || user_agent.contains("wget") || user_agent.contains("httpie") {
        return true;
    }

    return false;
}

#[get("/")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    let mut ip: String = String::from("");
    let headers = &req.app_data::<web::Data<Args>>().unwrap().lookup_headers;
    for header in headers {
        if let Some(value) = req.headers().get(header) {
            ip = value.clone().to_str().unwrap_or_default().to_owned();
            info!("ip: {}", ip);
            break;
        }
    }
    // If no ip could be resolved from upstream, an error shouuld be yielded
    if ip.is_empty() {
        error!("could not read IP for upstreams");
        return HttpResponse::NotAcceptable().body("");
    }
    // Allow requestors to display with text format
    if let Ok(params) = web::Query::<HashMap<String, String>>::from_query(req.query_string()) {
        if params.contains_key("bot") {
            return HttpResponse::Ok().body(ip);
        }
    }

    // Try to detect whether this request is comming from a spider, cli http client or web client.
    let is_bot = match req.headers().get("user-agent") {
        Some(v) => {
            let user_agent = v
                .clone()
                .to_str()
                .unwrap_or_default()
                .to_owned()
                .to_ascii_lowercase();
            request_is_bot(user_agent.as_str())
        }
        None => true,
    };

    if is_bot {
        return HttpResponse::Ok().body(ip);
    }

    let tpl = TemplateContext { ip_address_v4: ip };

    match tpl.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            error!("Could not render template due to: {}", e);
            HttpResponse::InternalServerError().body("an internal server error occurred")
        }
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let env = Env::new()
        .filter("BOTIP_LOG")
        .write_style("BOTIP_LOG_STYLE");
    env_logger::init_from_env(env);

    let args = Args::parse();
    args.inspect();

    let state = web::Data::new(args.clone());

    HttpServer::new(move || App::new().app_data(state.clone()).service(get_ip))
        .bind((args.bind_addr, args.bind_port))?
        .run()
        .await
}
