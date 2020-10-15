use hook::natives::*;

#[macro_export]
macro_rules! make_entrypoint {
    ($fn:expr) => {
        #[no_mangle]
        pub extern "stdcall" fn DllMain(
            _: winapi::shared::minwindef::HINSTANCE,
            reason: u32,
            _: *mut winapi::ctypes::c_void,
        ) {
            if reason == 1 {
                std::thread::spawn($fn);
            }
        }
    };
}

pub struct StreamedModel {
    hash: u32,
}

impl StreamedModel {
    pub fn new(hash: u32) -> StreamedModel {
        if hash != 0 {
            streaming::request_model(hash);

            while streaming::has_model_loaded(hash) == false {
                hook::script_wait(0);
            }
        }

        StreamedModel { hash }
    }
}

impl Drop for StreamedModel {
    fn drop(&mut self) {
        if self.hash == 0 {
            return;
        }

        streaming::set_model_as_no_longer_needed(self.hash);
    }
}
