use std::sync::Arc;
use walle_core::{
    action::Action,
    alt::TracingHandler,
    event::Event,
    obc::ImplOBC,
    OneBot,
    prelude::Resp,
};
use crate::onebot::MatrixHandler;

pub struct EventHandler {
    pub ob: Arc<OneBot<MatrixHandler, ImplOBC<Event>>>
}

impl EventHandler {
    pub fn init(ob: Arc<OneBot<MatrixHandler, ImplOBC<Event>>>) -> Self {
        Self { ob }
    }
}

impl Clone for EventHandler {
    fn clone(&self) -> Self {
        Self { ob: self.ob.clone() }
    }
}