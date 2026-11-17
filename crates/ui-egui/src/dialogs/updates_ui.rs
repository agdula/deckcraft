//! Application updates are managed centrally by the owner.
//!
//! Keep the host entry points as no-ops for compatibility, but never construct an
//! updater, contact a release service, display an update dialog, or self-replace
//! the executable. This preserves reviewed local customizations across updates.

use crate::SlideApp;

pub(crate) fn enable(_app: &mut SlideApp) {}

pub(crate) fn request_check(_app: &mut SlideApp) {}

pub(crate) fn start_pending_update(_app: &mut SlideApp) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centrally_managed_updates_do_not_open_a_dialog() {
        let mut app = SlideApp::new(deckcraft_engine::Session::new(), crate::Services::default());
        enable(&mut app);
        request_check(&mut app);
        assert!(app.dialog.is_none());
    }
}
