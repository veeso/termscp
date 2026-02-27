use tuirealm::props::{AttrValue, Attribute, Color, TableBuilder, TextSpan};

use super::super::{FileTransferActivity, Id, LogLevel, LogRecord};

const LOG_CAPACITY: usize = 256;

impl FileTransferActivity {
    /// Add message to log events
    pub(in crate::ui::activities::filetransfer) fn log(&mut self, level: LogLevel, msg: String) {
        // Log to file
        match level {
            LogLevel::Error => error!("{}", msg),
            LogLevel::Info => info!("{}", msg),
            LogLevel::Warn => warn!("{}", msg),
        }
        // Create log record
        let record: LogRecord = LogRecord::new(level, msg);
        //Check if history overflows the size
        if self.log_records.len() + 1 > LOG_CAPACITY {
            self.log_records.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.log_records.push_front(record);
        // Update log
        self.update_logbox();
        // flag redraw
        self.redraw = true;
    }

    /// Add message to log events and also display it as an alert
    pub(in crate::ui::activities::filetransfer) fn log_and_alert(
        &mut self,
        level: LogLevel,
        msg: String,
    ) {
        self.mount_error(msg.as_str());
        self.log(level, msg);
        // Update log
        self.update_logbox();
    }

    /// Update log box
    pub(in crate::ui::activities::filetransfer) fn update_logbox(&mut self) {
        let mut table: TableBuilder = TableBuilder::default();
        for (idx, record) in self.log_records.iter().enumerate() {
            // Add row if not first row
            if idx > 0 {
                table.add_row();
            }
            let fg = match record.level {
                LogLevel::Error => Color::Red,
                LogLevel::Warn => Color::Yellow,
                LogLevel::Info => Color::Green,
            };
            table
                .add_col(TextSpan::from(format!(
                    "{}",
                    record.time.format("%Y-%m-%dT%H:%M:%S%Z")
                )))
                .add_col(TextSpan::from(" ["))
                .add_col(
                    TextSpan::new(
                        format!(
                            "{:5}",
                            match record.level {
                                LogLevel::Error => "ERROR",
                                LogLevel::Warn => "WARN",
                                LogLevel::Info => "INFO",
                            }
                        )
                        .as_str(),
                    )
                    .fg(fg),
                )
                .add_col(TextSpan::from("]: "))
                .add_col(TextSpan::from(record.msg.as_str()));
        }
        assert!(
            self.app
                .attr(
                    &Id::Log,
                    Attribute::Content,
                    AttrValue::Table(table.build())
                )
                .is_ok()
        );
    }
}
