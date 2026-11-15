/// Runs a Future.
///
/// On native, it creates a second thread to be non-blocking.
///
/// On WASM, it is ran on the current thread, still non-blocking.
#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_future(fut: impl std::future::Future<Output = ()> + Send + 'static) {
    std::thread::spawn(move || pollster::block_on(fut));
}

/// Runs a Future.
///
/// On native, it creates a second thread to be non-blocking.
///
/// On WASM, it is ran on the current thread, still non-blocking.
#[cfg(target_arch = "wasm32")]
pub fn spawn_future(fut: impl std::future::Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}

/// Blocks this current thread to run a Future.
///
/// Unsupported on WASM.
#[allow(unused_variables)]
pub fn block_on<F>(fut: F) -> Result<F::Output, crate::error::Error>
where
    F: std::future::Future,
{
    #[cfg(target_arch = "wasm32")]
    {
        Err(crate::error::Error::DeviceLost)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(pollster::block_on(fut))
    }
}
