use std::{fmt::Debug, time::SystemTime};

use log::trace;
use time::{macros::format_description, OffsetDateTime};
use tokio::sync::mpsc::{error::SendError as TokioSendError, UnboundedSender};

use crate::profiling::tracy_dynamic_zone;

#[derive(Clone, Debug)]
pub struct LoggingSender<T>
where
    T: Debug + AsRef<str>,
{
    tx: UnboundedSender<T>,
    channel_name: String,
}

impl<T> LoggingSender<T>
where
    T: Debug + AsRef<str>,
{
    pub fn attach(tx: UnboundedSender<T>, channel_name: &str) -> Self {
        Self {
            tx,
            channel_name: channel_name.to_string(),
        }
    }

    pub fn send(&self, message: T) -> Result<(), TokioSendError<T>> {
        tracy_dynamic_zone!(&format!("{}::{}", self.channel_name, message.as_ref()));

        let mes = format!("{} {:?}", self.channel_name, &message);

        if mes.contains("BS") {
            let system_time: OffsetDateTime = SystemTime::now().into();
            let timestamp = system_time
                .format(format_description!("[second].[subsecond digits:3]"))
                .expect("Failed to parse current time");

            log::error!(
                "UICommand send{:?} {} {:?}",
                timestamp,
                self.channel_name,
                &message
            );
        }

        trace!("{} {:?}", self.channel_name, &message);
        self.tx.send(message)
    }
}
