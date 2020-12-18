use crate::transform::TransformError;
use anyhow::Context;
use clap::ArgMatches;
use warp::http::Response;
use warp::hyper::Body;
use warp::reply::Reply;
use warp::Filter;

pub async fn dev(matches: &ArgMatches<'_>) -> anyhow::Result<()> {
    let route = warp::fs::dir(std::env::current_dir().unwrap()).and_then(
        |reply: warp::fs::File| async move {
            match crate::transform::transform(reply.path()).await {
                Ok(contents) => Ok(Response::builder()
                    .header("content-type", "text/javascript")
                    .body(Body::from(contents)).unwrap()),
                Err(TransformError::UnsupportedType) => Ok(reply.into_response()),
                Err(TransformError::IoError(_)) => Err(warp::reject()),
            }
        },
    );

    warp::serve(route)
        .run((
            [127, 0, 0, 1],
            matches
                .value_of("port")
                .unwrap_or("8080")
                .parse()
                .with_context(|| "Invalid port")?,
        ))
        .await;

    Ok(())
}
