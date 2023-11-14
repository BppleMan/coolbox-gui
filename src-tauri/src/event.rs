use cool::channel::{Receiver, Sender};
use cool::{channel, info, TaskEvent};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;
use tauri::Window;

#[allow(clippy::type_complexity)]
static EVENT_LOOP_CHANNEL: Lazy<(
    Mutex<Sender<MainWindowEvent>>,
    Mutex<Receiver<MainWindowEvent>>,
)> = Lazy::new(|| {
    let (tx, rx) = channel::unbounded();
    (Mutex::new(tx), Mutex::new(rx))
});

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MainWindowEvent {
    AskPass,
    TaskEvent(TaskEvent),
}

impl MainWindowEvent {
    pub fn name(&self) -> &'static str {
        match self {
            MainWindowEvent::AskPass => "ask_pass",
            MainWindowEvent::TaskEvent(_) => "task_event",
        }
    }
}

impl Display for MainWindowEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MainWindowEvent::AskPass => write!(f, "ask_pass"),
            MainWindowEvent::TaskEvent(task_event) => write!(f, "task_event: {}", task_event),
        }
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn start_event_loop(main_window: Window) {
        info!("Starting Event Loop");
        let receiver = EVENT_LOOP_CHANNEL.1.lock().unwrap().clone();
        while let Ok(event) = receiver.recv() {
            let event_name = event.name();
            match event {
                MainWindowEvent::AskPass => {
                    main_window.emit(event_name, EmptyPayload).unwrap();
                }
                MainWindowEvent::TaskEvent(task_event) => {
                    main_window.emit(event_name, task_event).unwrap();
                }
            }
        }
    }

    pub fn ask_pass() {
        EVENT_LOOP_CHANNEL
            .0
            .lock()
            .unwrap()
            .send(MainWindowEvent::AskPass)
            .unwrap();
    }

    pub fn task_event(task_event: TaskEvent) {
        EVENT_LOOP_CHANNEL
            .0
            .lock()
            .unwrap()
            .send(MainWindowEvent::TaskEvent(task_event))
            .unwrap();
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyPayload;
