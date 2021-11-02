use serde_json::json;
use warp::{reply::Json, Filter};

pub fn todos_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// LIST todos
	warp::path("todos")
		.and(warp::get())
		.and(warp::path::end())
		.and_then(todo_list)
}

async fn todo_list() -> Result<Json, warp::Rejection> {
	// TODO - get from DB
	let todos = json!([
		{"id": 1, "title": "todo 1"},
		{"id": 2, "title": "todo 2"}
	]);

	let todos = warp::reply::json(&todos);

	Ok(todos)
}
