//! Help ▸ Check for updates: this copy of NOVA PDF never goes online.
//!
//! The NOVA Workspace suite is shipped as one complete bundle, so there is
//! nothing to fetch over the network. The command still works; it only says
//! that this copy does not check for updates, and no address is ever contacted.

use crate::NovaPdfApp;

pub(crate) struct Updates {
    /// The Updates dialog is showing.
    pub(crate) open: bool,
    /// A check was asked for; the local note is pending.
    check_requested: bool,
}

impl Default for Updates {
    fn default() -> Self {
        Self { open: false, check_requested: false }
    }
}

impl NovaPdfApp {
    /// Whether the app asks for a newer release on its first frame. It never does.
    pub fn set_startup_update_check(&mut self, _on: bool) {}

    /// Help ▸ Check for updates: says so locally instead of going out to the network.
    pub fn check_for_updates(&mut self) {
        self.updates.open = true;
        self.updates.check_requested = true;
    }

    /// Starts the checks and picks up their answers (each frame).
    pub(crate) fn poll_updates(&mut self) {
        if std::mem::take(&mut self.updates.check_requested) {
            self.updates.open = false;
            self.notify("This copy of NOVA PDF does not check for updates. The NOVA Workspace suite is installed as one bundle.");
        }
    }
}

/// The Updates window: there is nothing to show, the note goes to the status bar.
pub(crate) fn dialog(app: &mut NovaPdfApp, _ctx: &egui::Context) {
    app.updates.open = false;
}
