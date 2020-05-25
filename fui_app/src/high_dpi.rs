#![allow(non_snake_case)]

#[cfg(windows)]
use std::result;

#[cfg(windows)]
use std::mem;

/// Makes the process high-DPI aware.
///
/// Without high-DPI awareness, a process' window would be scaled by Windows on high-DPI display.
/// This can cause blurry text and degrade the quality of images.
/// A well-written application would handle high-DPI displays by adapting its layout and using bigger fonts.
///
/// Note: it's recommended to mark your applications as high-DPI aware by embedding an
/// [application manifest](https://msdn.microsoft.com/en-us/library/windows/desktop/aa374191(v=vs.85).aspx).
///
/// For more information read [Writing High-DPI aware applications](https://msdn.microsoft.com/en-us/library/windows/desktop/dn469266(v=vs.85).aspx).
#[cfg(windows)]
pub fn set_process_high_dpi_aware() {
    // First, try setting the high-DPI awareness using the Windows 10 and newer API.
    let _result = set_high_dpi_windows_10()
        // Then try the Windows 8 way.
        .or_else(|_| set_high_dpi_windows_8())
        // Fall back to the old, Windows Vista method.
        .or_else(|_| set_high_dpi_windows_vista());

    // If all methods failed, either the OS is older than Vista
    // or the user already set the process as high-DPI aware using an application manifest.
}

/// This function only works on Windows.
#[cfg(not(windows))]
pub fn set_process_high_dpi_aware() {}

#[cfg(windows)]
type Result = result::Result<(), ()>;

// Helper function to dynamically load a function pointer and call it.
// The result of the callback is forwarded.
#[cfg(windows)]
fn try_get_function_pointer<F>(dll: &str, name: &str, callback: &Fn(&F) -> Result) -> Result {
    use shared_library::dynamic_library::DynamicLibrary;
    use std::path::Path;

    // Try to load the function dynamically.
    let lib = DynamicLibrary::open(Some(Path::new(dll))).map_err(|_| ())?;

    let func_ptr = unsafe { lib.symbol::<F>(name).map_err(|_| ())? };

    let func = unsafe { mem::transmute(&func_ptr) };

    callback(func)
}

// Tries to set the application as per-monitor high-DPI aware.
// Uses the `SetProcessDpiAwarenessContext` function.
// See https://msdn.microsoft.com/en-us/library/windows/desktop/mt807676(v=vs.85).aspx
#[cfg(windows)]
fn set_high_dpi_windows_10() -> Result {
    try_get_function_pointer::<unsafe extern "system" fn(context: usize) -> u32>(
        "User32.dll",
        "SetProcessDpiAwarenessContext",
        &|SetProcessDpiAwarenessContext| {
            // First, try using the new Per-Monitor high-DPI awareness introduced in Windows 10 Creators Update,
            // to benefit from the added features.
            // See https://msdn.microsoft.com/library/windows/desktop/mt791579(v=vs.85).aspx for reference.
            let DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: usize =
                unsafe { mem::transmute(-4 as isize) };

            let result = unsafe {
                SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
            };

            match result {
                // The V2 only works with Windows 10 Creators Update. Try using the older per-monitor context V1.
                0 => {
                    let DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE =
                        unsafe { mem::transmute(-3 as isize) };
                    let result = unsafe {
                        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE)
                    };

                    match result {
                        0 => Err(()),
                        _ => Ok(()),
                    }
                }
                _ => Ok(()),
            }
        },
    )
}

// Tries to set the application as per-monitor high-DPI aware.
// Uses the `SetProcessDpiAwareness` function.
// See https://msdn.microsoft.com/en-us/library/windows/desktop/dn302122(v=vs.85).aspx
#[cfg(windows)]
fn set_high_dpi_windows_8() -> Result {
    try_get_function_pointer::<unsafe extern "system" fn(u32) -> u32>(
        "Shcore.dll",
        "SetProcessDpiAwareness",
        &|SetProcessDpiAwareness| {
            // From https://msdn.microsoft.com/en-us/library/windows/desktop/dn280512(v=vs.85).aspx
            const PROCESS_PER_MONITOR_DPI_AWARE: u32 = 2;

            let result = unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE) };

            match result {
                0 => Ok(()),
                _ => Err(()),
            }
        },
    )
}

// Tries to set the application as system high-DPI aware.
// Uses the old `SetProcessDPIAware` function.
#[cfg(windows)]
fn set_high_dpi_windows_vista() -> Result {
    try_get_function_pointer::<unsafe extern "system" fn() -> u32>(
        "User32.dll",
        "SetProcessDPIAware",
        &|SetProcessDPIAware| {
            // See https://msdn.microsoft.com/en-us/library/windows/desktop/ms633543(v=vs.85).aspx
            let result = unsafe { SetProcessDPIAware() };

            match result {
                0 => Err(()),
                _ => Ok(()),
            }
        },
    )
}
