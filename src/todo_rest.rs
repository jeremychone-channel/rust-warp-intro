use warp::Filter;

pub fn todos_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	// LIST todos
	warp::path("todos")
		.and(warp::get())
		.and(warp::path::end())
		.map(|| "will get todos")
}
