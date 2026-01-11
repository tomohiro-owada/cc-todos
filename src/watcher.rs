use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

pub enum WatchEvent {
    Changed,
}

pub struct DirWatcher {
    _watcher: RecommendedWatcher,
    pub rx: mpsc::Receiver<WatchEvent>,
}

impl DirWatcher {
    pub fn new(dir: &PathBuf) -> Result<Self> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    if event.kind.is_modify() || event.kind.is_create() {
                        let _ = tx.send(WatchEvent::Changed);
                    }
                }
            },
            Config::default().with_poll_interval(Duration::from_millis(100)),
        )?;

        watcher.watch(dir, RecursiveMode::NonRecursive)?;

        Ok(DirWatcher {
            _watcher: watcher,
            rx,
        })
    }

    pub fn try_recv(&self) -> Option<WatchEvent> {
        self.rx.try_recv().ok()
    }
}
