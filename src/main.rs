use futures_util::{StreamExt, FutureExt, SinkExt};
use warp::Filter;

mod synch;
mod ws;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    tokio::spawn(async {
        loop { // loop to ensure it never stops
            synch::manage_htcal().await;
        }
    });

    // serve ws, and then serve static files

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(ws::websocket)
        })
        .or(warp::fs::dir("static"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}