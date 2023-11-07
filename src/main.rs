#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tool_slint::{datetime, get_result, *, sql::login_user};
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let app = App::new().unwrap();
    let cargo_worker = get_result::CargoWorker::new(&app);
    let _timer = datetime::setup(&app);
    let weak_window = app.as_weak();
    app.on_login_clicked({
        move |logininfos| {
            let r = run_login(logininfos);
            weak_window.unwrap().set_is_login(r);
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
fn run_login(logininfos: LoginInfos) -> bool {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
    let r = login_user(logininfos.name.to_string(), logininfos.password.to_string()).await;
    r
    });
    res
    
}