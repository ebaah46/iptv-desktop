use qtbridge::{QmlObject, qobject};

use crate::core::context;

/// Application context that provide access to run startup functions
/// and also to to register application wide error states.
pub struct AppState {
    ready: bool,
    error: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            ready: false,
            error: String::new(),
        }
    }
}

#[qobject(ConvertToCamelCase)]
impl AppState {
    qproperty!("ready", Member = ready, Notify = ready_changed);
    qproperty!("error", Member = error, Notify = error_changed);

    #[qsignal(qml_name = "readyChanged")]
    fn ready_changed(&mut self);

    #[qsignal(qml_name = "errorChanged")]
    fn error_changed(&mut self);

    #[qslot]
    fn load(&mut self) {
        let handle = self.get_thread_handle();
        context::get().spawn_blocking(move || {
            // `load` reports failures through `Result`; the panic guard is a
            // safety net so a core panic cannot leave the UI on the loading screen.
            let result = std::panic::catch_unwind(|| context::get().load())
                .unwrap_or_else(|_| Err(anyhow::anyhow!("catalog load panicked")));
            handle.invoke_mut(move |state| match result {
                Ok(outcome) => {
                    log::info!("Catalog loaded from {:?}", outcome.source);
                    state.ready = true;
                    state.ready_changed();
                }
                Err(e) => {
                    state.error = format!("Failed to load catalog: {e}");
                    state.error_changed();
                }
            });
        });
    }
}
