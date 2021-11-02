use warp::Filter;

#[tokio::main]
async fn main() {
	let hello_world = warp::path::end().map(|| "Hello world from root!");

	let hi = warp::path("hi").map(|| "Hello from hi");

	let routes = hello_world.or(hi);

	println!("start web-server");
	warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
