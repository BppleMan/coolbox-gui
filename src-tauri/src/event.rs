use cool::channel::{Receiver, Sender};
use cool::{channel, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use tauri::Window;

#[allow(clippy::type_complexity)]
static ASK_PASS_EVENT_CHANNEL: Lazy<(
    Mutex<Sender<MainWindowEvent>>,
    Mutex<Receiver<MainWindowEvent>>,
)> = Lazy::new(|| {
    let (tx, rx) = channel::unbounded();
    (Mutex::new(tx), Mutex::new(rx))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MainWindowEvent {
    AskPass,
}

impl Display for MainWindowEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MainWindowEvent::AskPass => write!(f, "ask_pass"),
        }
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn start_event_loop(main_window: Window) {
        info!("Starting Event Loop");
        let receiver = ASK_PASS_EVENT_CHANNEL.1.lock().unwrap().clone();
        while let Ok(event) = receiver.recv() {
            match event {
                MainWindowEvent::AskPass => {
                    main_window
                        .emit(&format!("{}", event), EmptyPayload)
                        .unwrap();
                }
            }
        }
    }

    pub fn ask_pass() {
        ASK_PASS_EVENT_CHANNEL
            .0
            .lock()
            .unwrap()
            .send(MainWindowEvent::AskPass)
            .unwrap();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyPayload;
