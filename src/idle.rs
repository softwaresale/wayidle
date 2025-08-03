use log::{error, info};
use wayland_client::Dispatch;
use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::ExtIdleNotificationV1;

use crate::{
    app::{WayIdleApp, WayIdleUserState},
    command::execute_command,
};

#[derive(Default)]
pub struct IdleListener;

impl Dispatch<ExtIdleNotificationV1, WayIdleUserState, WayIdleApp> for IdleListener {
    fn event(
        _state: &mut WayIdleApp,
        _proxy: &ExtIdleNotificationV1,
        event: <ExtIdleNotificationV1 as wayland_client::Proxy>::Event,
        data: &WayIdleUserState,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<WayIdleApp>,
    ) {
        match event {
            wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::Event::Idled => {
                info!("system is idled");
                match execute_command(data.command()) {
                    Ok(_) => info!("successfully invoked idle command"),
                    Err(err) => error!("error while invoking idle command: {}", err),
                }
            },
            wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::Event::Resumed => {
                info!("system has resumed");
            },
            _ => {},
        }
    }
}
