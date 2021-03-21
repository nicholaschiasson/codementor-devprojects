use actix_files::NamedFile;
use actix_web::{guard, middleware::Logger, web, App, HttpRequest, HttpServer, Result};
use env_logger::Env;
use std::path::PathBuf;

const HOST: &str = "simple-web-contact-form.codementor-devprojects.com";

async fn static_file(req: HttpRequest) -> Result<NamedFile> {
	let mut path = PathBuf::from("./frontend/static");
	path.push(
		req
			.match_info()
			.query("filename")
			.parse::<String>()
			.unwrap(),
	);
	Ok(NamedFile::open(path)?)
}

async fn index(_: HttpRequest) -> Result<NamedFile> {
	Ok(NamedFile::open(PathBuf::from(
		"./frontend/static/index.html",
	))?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(|| {
		App::new()
			.wrap(Logger::default())
			.service(
				web::resource("/{filename:.*\\.(css|ico|js|map|png|svg|wasm)$}")
					.guard(guard::Host(HOST))
					.route(web::get().to(static_file)),
			)
			.service(
				web::resource("/|.*")
					.guard(guard::Host(HOST))
					.route(web::get().to(index))
					.route(web::post().to(index)),
			)
	})
	.bind("0.0.0.0:3000")?
	.run()
	.await
}
