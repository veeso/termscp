//! ## Pending actions
//!
//! this little module exposes the routine to create a pending action on the file transfer activity.
//! A pending action is an action which blocks the execution of the application in await of a certain `Msg`.

use tuirealm::{PollStrategy, Update};

use super::{FileTransferActivity, Msg};

impl FileTransferActivity {
    /// Block execution of activity, preventing ANY kind of message not specified in the `wait_for` argument.
    /// Once `wait_for` clause is satisfied, the function returns.
    ///
    /// Returns the message which satisfied the clause
    ///
    /// NOTE: The view is redrawn as usual
    pub(super) fn wait_for_pending_msg(&mut self, wait_for: &[Msg]) -> Msg {
        self.redraw = true;
        loop {
            // Poll
            match self.app.tick(PollStrategy::Once) {
                Ok(mut messages) => {
                    if !messages.is_empty() {
                        self.redraw = true;
                    }
                    let found = messages.iter().position(|m| wait_for.contains(m));
                    // Return if found
                    if let Some(index) = found {
                        return messages.remove(index);
                    } else {
                        // Update
                        for msg in messages.into_iter() {
                            let mut msg = Some(msg);
                            while msg.is_some() {
                                msg = self.update(msg);
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("Application error: {}", err);
                }
            }
            // Redraw
            if self.redraw {
                self.view();
            }
        }
    }
}
