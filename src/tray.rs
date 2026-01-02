pub mod tray {

    use iced::{self, Subscription, futures::{SinkExt, Stream, channel::mpsc::Sender}};
    use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
    use std::time::Duration;
    use crate::state::MainMessage;

    pub fn tray_icon(icon_raw: &[u8]) -> TrayIcon {
        let icon_image = image
            ::load_from_memory(icon_raw)
            .expect("No tray icon image found.")
            .to_rgba8();

        let (width, height) = icon_image.dimensions();

        TrayIconBuilder::new()
            .with_tooltip("EyeCatcher")
            .with_icon(Icon::from_rgba(icon_image.into_raw(), width, height).expect("Could not instantiate icon image for tray."))
            .build()
            .expect("Could not start tray icon.")
    }

    pub fn tray_stream() -> impl Stream<Item = MainMessage> {
        iced::stream::channel(
            100,
            |mut output: Sender<MainMessage>| async move {
                let receiver = TrayIconEvent::receiver();

                let mut interval = tokio::time::interval(Duration::from_millis(100));

                loop {
                    interval.tick().await;
                    // TODO: do events
                    while let Ok(_) = receiver.try_recv() {
                        let _ = output.send(MainMessage::TrayEvent).await;
                    }
                }
            }   
        )
    }

    pub fn tray_subscription() -> Subscription<MainMessage> {
        Subscription::run(tray_stream)
    }
}