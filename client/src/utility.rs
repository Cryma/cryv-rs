#[macro_export]
macro_rules! make_entrypoint {
    ($fn:expr) => {
        #[no_mangle]
        pub extern "stdcall" fn DllMain(_: winapi::shared::minwindef::HINSTANCE, reason: u32, _: *mut winapi::ctypes::c_void) {
            if reason == 1 {
                std::thread::spawn($fn);
            }
        }
    };
}

#[macro_export]
macro_rules! ptr {
    ($address:expr, $type:ty) => {
        *($address as *mut $type)
    };
}
