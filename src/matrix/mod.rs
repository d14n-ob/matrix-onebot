mod entry;
mod unsafe_actions;
mod handlers;

pub use crate::matrix::entry::{
    add_event_handlers,
    create_client,
};

pub use handlers::EventHandler;