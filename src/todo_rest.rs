use warp::Filter;

pub fn todos_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// LIST todos
	warp::path("todos")
		.and(warp::get())
		.and(warp::path::end())
		.and_then(todo_list)
}

async fn todo_list() -> Result<String, warp::Rejection> {
	Ok("Will list todos".to_string())
}
