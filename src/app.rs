use std::time::Duration;

use log::info;
use wayland_client::{
    Dispatch, delegate_dispatch, delegate_noop,
    protocol::{wl_registry, wl_seat::WlSeat},
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::ExtIdleNotificationV1, ext_idle_notifier_v1::ExtIdleNotifierV1,
};

use crate::idle::IdleListener;

#[derive(Clone, Copy)]
pub struct WayIdleUserState {
    pub(crate) idle_duration: Duration,
}

#[derive(Default)]
pub struct WayIdleApp {
    idle_listener: IdleListener,
    config: IdleConfig,
}

#[derive(Default)]
struct IdleConfig {
    seat: Option<WlSeat>,
    idle_notifier: Option<ExtIdleNotifierV1>,
    active: bool,
}

impl IdleConfig {
    fn add_seat(&mut self, incoming_seat: WlSeat) {
        self.seat = Some(incoming_seat);
    }

    fn add_notifier(&mut self, incoming_notifier: ExtIdleNotifierV1) {
        self.idle_notifier = Some(incoming_notifier);
    }

    fn ready(&self) -> Option<(&WlSeat, &ExtIdleNotifierV1)> {
        match (self.seat.as_ref(), self.idle_notifier.as_ref()) {
            (Some(seat), Some(notifier)) => Some((seat, notifier)),
            _ => None,
        }
    }
}

impl AsMut<IdleListener> for WayIdleApp {
    fn as_mut(&mut self) -> &mut IdleListener {
        &mut self.idle_listener
    }
}

impl Dispatch<wl_registry::WlRegistry, WayIdleUserState> for WayIdleApp {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        data: &WayIdleUserState,
        _conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                info!(
                    "global registered: name={}, interface={}, version={}",
                    name, interface, version
                );

                match &interface[..] {
                    "wl_seat" => {
                        let seat = proxy.bind::<WlSeat, _, _>(name, version, qhandle, ());
                        state.config.add_seat(seat);
                    }

                    "ext_idle_notifier_v1" => {
                        info!("found idle notifier");
                        let idle_notifier =
                            proxy.bind::<ExtIdleNotifierV1, _, _>(name, version, qhandle, ());

                        state.config.add_notifier(idle_notifier);
                    }
                    _ => {}
                }

                // if all necessary data is bound, then we can setup our idle_notification
                if let Some((seat, notifier)) = state.config.ready()
                    && !state.config.active
                {
                    notifier.get_idle_notification(
                        data.idle_duration.as_millis() as u32,
                        seat,
                        qhandle,
                        *data,
                    );

                    info!("configured idle notification");
                    state.config.active = true;
                }
            }
            wl_registry::Event::GlobalRemove { name } => {
                info!("global removed: name={}", name)
            }
            _ => {}
        }
    }
}

delegate_noop!(WayIdleApp: ignore WlSeat);
delegate_noop!(WayIdleApp: ignore ExtIdleNotifierV1);
delegate_dispatch!(WayIdleApp: [ExtIdleNotificationV1: WayIdleUserState] => IdleListener);
