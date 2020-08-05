use actix_web::{get, web, App as wApp, HttpServer, Responder};
use clap::App;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    //get matches
    let matches = App::new("Thinkphp Dashboard")
        .version("0.1.0")
        .author("Bison 'goldeagle' Fan <bison@bitseed.cn>")
        .about("A dashboard for manage projects using thinkphp v6.0 framework. https://github.com/thinkphp-tech/thinkphp-dash")
        .arg("-c, --config=[FILE] 'Sets a coustom config file'")
        .arg("--rpc 'Run as rpc mode'")
        .arg("-h, --host=[HOST] 'Bind daemon with $HOST'")
        .arg("-p, --port=[PORT] 'Bind daemon with $PORT'")
        .get_matches();

    //process args
    let mut host = "127.0.0.1";
    let mut port = "10000";

    //arg:host
    if matches.is_present("host") {
        if let Some(h) = matches.value_of("host") {
            host = h;
        }
    }

    //arg:port
    if matches.is_present("port") {
        if let Some(p) = matches.value_of("port") {
            port = p;
        }
    }

    HttpServer::new(|| wApp::new().service(index))
        .bind(format!("{}:{}", String::from(host), String::from(port)))?
        .run()
        .await
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}
