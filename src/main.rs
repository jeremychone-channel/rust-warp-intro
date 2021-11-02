use warp::Filter;

const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {
	// APIs
	let hi = warp::path("hi").and(warp::get()).map(|| "Hello from hi");
	let apis = hi;

	// Static Content
	let content = warp::fs::dir(WEB_FOLDER);
	let root = warp::get()
		.and(warp::path::end())
		.and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
	let static_site = content.or(root);

	let routes = apis.or(static_site);

	println!("start web-server");
	warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
