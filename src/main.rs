use std::sync::Arc;
use futures_util::{StreamExt, FutureExt, SinkExt};
use handlebars::Handlebars;
use serde::Serialize;
use warp::Filter;

pub mod synch;
mod ws;
mod calendar;

// also templates here
pub struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
    where
        T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();

    tokio::spawn(async {
        loop { // loop to ensure it never stops
            synch::manage_htcal().await;
        }
    });

    // load templates
    let mut hbs = Handlebars::new();
    hbs.register_template_file("calendar", "templates/calendar.html")
        .unwrap();
    let hbs = Arc::new(hbs);

    let handlebars = move |with_template| render(with_template, hbs.clone());

    // serve ws, and then serve static files

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(ws::websocket)
        })
        .or(warp::path("calendar")
            .and(warp::get())
            .map(|| calendar::route())
            .map(handlebars)
        )
        .or(warp::fs::dir("static"));

    warp::serve(routes).run(([0, 0, 0, 0], 7070)).await;

    Ok(())
}