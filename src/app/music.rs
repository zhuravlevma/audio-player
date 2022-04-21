use std::sync::Arc;
use crate::app::handler::Handler;

pub struct Music {
    handler: Handler
}

impl Music {
    pub fn new() -> Self {
        Self {
            handler: Handler::new(),
        }
    }

    pub fn new_track(&self) -> rodio::Sink {
        self.handler.create_sink()
    }

    pub fn replace_track(&self, sink: &Arc<rodio::Sink>) -> rodio::Sink {
        self.stop_track(sink);
        self.new_track()
    }

    pub fn stop_track(&self, mut sink: &Arc<rodio::Sink>) -> bool {
        sink.stop();
        true
    }
}