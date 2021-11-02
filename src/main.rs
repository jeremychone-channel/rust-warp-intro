use warp::Filter;

#[tokio::main]
async fn main() {
	let hello_world = warp::path::end().map(|| "Hello world from root!");

	println!("start web-server");
	warp::serve(hello_world).run(([127, 0, 0, 1], 8080)).await;
}
