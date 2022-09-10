use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use botip::args::Args;
use std::io::Result;

#[get("/")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    let headers = &req.app_data::<web::Data<Args>>().unwrap().lookup_headers;
    for header in headers {
        if let Some(value) = req.headers().get(header) {
            return HttpResponse::Ok().body(value.clone().to_str().unwrap_or_default().to_owned());
        }
    }
    return HttpResponse::NotAcceptable().body("");
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    args.inspect();

    let state = web::Data::new(args.clone());

    HttpServer::new(move || App::new().app_data(state.clone()).service(get_ip))
        .bind((args.bind_addr, args.bind_port))?
        .run()
        .await
}
