mod filelist;
mod host;
mod log;
mod notify;

use tuirealm::{PollStrategy, Update};

use super::FileTransferActivity;

impl FileTransferActivity {
    /// Call `Application::tick()` and process messages in `Update`
    pub(super) fn tick(&mut self) {
        match self.app.tick(PollStrategy::UpTo(1)) {
            Ok(messages) => {
                if !messages.is_empty() {
                    self.redraw = true;
                }
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                }
            }
            Err(err) => {
                self.mount_error(format!("Application error: {err}"));
            }
        }
    }
}
