use crossbeam::channel::{SendError};
use log::error;
use std::error::Error;

use crate::{entity::EventMessage, global_variables::EVENT_CHANNEL};

pub type SuperError = Box<dyn Error>;

/// Handle result function of methods, process exception if any, return content if there's no exception
pub fn handle_result<T>(result: Result<T, SuperError>) -> Option<T> {
    match result {
        Ok(t) => Some(t),
        Err(e) => {
            handle_send_result(EVENT_CHANNEL.0.send(EventMessage::ShowError(e.to_string())));
            None
        }
    }
}

pub fn handle_send_result<T>(result: Result<(), SendError<T>>) {
    if let Err(err) = result {
        error!("{}", err);
    }
}
