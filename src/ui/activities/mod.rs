//! ## Activities
//!
//! `activities` is the module which provides all the different activities
//! each activity identifies a layout with its own logic in the UI

// Locals
use super::context::Context;
// Activities
pub mod auth;
pub mod filetransfer;
pub mod setup;

// -- Exit reason

pub enum ExitReason {
    Quit,
    Connect,
    Disconnect,
    EnterSetup,
}

// -- Activity trait

pub trait Activity {
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context);

    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self);

    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason>;

    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context>;
}
