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
            let result = std::panic::catch_unwind(|| context::get().refresh());
            handle.invoke_mut(move |state| match result {
                Ok(()) => {
                    state.ready = true;
                    state.ready_changed();
                }
                Err(_) => {
                    state.error = "Failed to load catalog".into();
                    state.error_changed();
                }
            });
        });
    }
}
