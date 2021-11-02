use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn do_auth(
) -> impl Filter<Extract = (UserCtx,), Error = warp::Rejection> + Clone {
	warp::any()
		.and(warp::header::<String>(HEADER_XAUTH))
		.and_then(|xauth: String| async move {
			// Check auth
			if !xauth.ends_with(".exp.signature") {
				return Err(warp::reject::custom(FailAuth));
			}

			if let Some(user_id) = xauth
				.splitn(2, ".")
				.next()
				.and_then(|v| v.parse::<i64>().ok())
			{
				Ok::<UserCtx, warp::Rejection>(UserCtx { user_id })
			} else {
				Err(warp::reject::custom(FailAuth))
			}
		})
}

pub struct UserCtx {
	pub user_id: i64,
}

#[derive(Debug)]
pub struct FailAuth;

impl warp::reject::Reject for FailAuth {}
