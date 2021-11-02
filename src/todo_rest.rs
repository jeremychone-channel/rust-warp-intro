use serde_json::json;
use warp::{reply::Json, Filter};

pub fn todos_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let todos_base = warp::path("todos");
	// LIST todos
	let list = todos_base
		.and(warp::get())
		.and(warp::path::end())
		.and_then(todo_list);

	let get = todos_base
		.and(warp::get())
		.and(warp::path::param()) // e.g., /todos/123
		.and_then(todo_get);

	list.or(get)
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

async fn todo_get(id: i64) -> Result<Json, warp::Rejection> {
	// TODO - get from DB
	let todo = json!(
		{"id": id, "title": format!("todo {}", id)}
	);

	// serde-json to warp-reply
	let todo = warp::reply::json(&todo);

	Ok(todo)
}
