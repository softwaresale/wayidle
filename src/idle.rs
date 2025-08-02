use log::info;
use wayland_client::Dispatch;
use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::ExtIdleNotificationV1;

use crate::app::{WayIdleApp, WayIdleUserState};

#[derive(Default)]
pub struct IdleListener;

impl Dispatch<ExtIdleNotificationV1, WayIdleUserState, WayIdleApp> for IdleListener {
    fn event(
        _state: &mut WayIdleApp,
        _proxy: &ExtIdleNotificationV1,
        event: <ExtIdleNotificationV1 as wayland_client::Proxy>::Event,
        _data: &WayIdleUserState,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<WayIdleApp>,
    ) {
        match event {
            wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::Event::Idled => {
                info!("system is idled");
            },
            wayland_protocols::ext::idle_notify::v1::client::ext_idle_notification_v1::Event::Resumed => {
                info!("system has resumed");
            },
            _ => {},
        }
    }
}
