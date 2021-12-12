//! ## Pending actions
//!
//! this little module exposes the routine to create a pending action on the file transfer activity.
//! A pending action is an action which blocks the execution of the application in await of a certain `Msg`.

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use super::{FileTransferActivity, Msg};

use tuirealm::PollStrategy;

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
                Ok(messages) => {
                    if !messages.is_empty() {
                        self.redraw = true;
                    }
                    if let Some(msg) = messages.into_iter().find(|m| wait_for.contains(m)) {
                        return msg;
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
