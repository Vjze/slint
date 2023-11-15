#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod logic;
pub use logic::*;
#[allow(clippy::all)]
pub mod generated_code {
    slint::include_modules!();
}
pub use generated_code::*;
use logic::sql::login_user;
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
#[tokio::main]
async fn main() {
    let app = App::new().unwrap();
    let cargo_worker = get_result::CargoWorker::new(&app);
    let _timer = datetime::setup(&app);
    let weak_window = app.as_weak();
    app.on_login_clicked({
        move |logininfos| {
            let _ = weak_window
            .upgrade_in_event_loop(move |ui| {
                slint::spawn_local(async move{
                    let r = login_user(logininfos.name.to_string(), logininfos.password.to_string()).await;
                    ui.set_is_login(r);
                }).unwrap();
            });
        }

    });
    let weak_window = app.as_weak();
    app.on_login_cancel(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });
    app.global::<InfosData>().on_query({
        let window = app.as_weak();
        let cargo_channel = cargo_worker.channel.clone();
        move |action| {
            if action.text.len() == 0 {
                window.unwrap().invoke_show_confirm_popup();
            } else {
                cargo_channel
                    .send(get_result::QueryMessage::Action { action })
                    .unwrap()
            }
        }
    });
    let window = app.as_weak();
    window
        .unwrap()
        .global::<InfosData>()
        .set_version(APP_VERSION.into());

    app.run().unwrap();
    cargo_worker.join().unwrap();
}
