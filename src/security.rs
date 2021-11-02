use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn check_auth(
) -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
	warp::any()
		.and(warp::header::<String>(HEADER_XAUTH))
		.and_then(|xauth: String| async move {
			// Check auth
			if !xauth.ends_with(".exp.signature") {
				panic!("FIX ME - SHOULD NOT PANIC");
			}

			Ok::<(), warp::Rejection>(())
		})
}
