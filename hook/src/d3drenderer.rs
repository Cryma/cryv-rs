use cpp::cpp;
use winapi::Interface;

use crate::POINTERS;

pub type D3DCallback =
    fn(*mut winapi::um::d3d11::ID3D11Device, *mut winapi::um::d3d11::ID3D11DeviceContext);

struct RendererData {
    device: Option<*mut winapi::um::d3d11::ID3D11Device>,
    context: Option<*mut winapi::um::d3d11::ID3D11DeviceContext>,
    present_callbacks: Vec<D3DCallback>,
    // pre_resize_callbacks: Vec<D3DCallback>,
    // post_resize_callbacks: Vec<D3DCallback>,
}

impl RendererData {
    const INIT: Self = RendererData {
        device: None,
        context: None,
        present_callbacks: Vec::new(),
        // pre_resize_callbacks: Vec::new(),
        // post_resize_callbacks: Vec::new(),
    };
}

static mut RENDERER_DATA: RendererData = RendererData::INIT;

#[derive(Copy, Clone)]
pub struct VmtHook {
    object: *mut std::ffi::c_void,
}

impl VmtHook {
    pub fn new(hook_object: *mut std::ffi::c_void, function_amount: isize) -> VmtHook {
        unsafe {
            let object = cpp!([hook_object as "void*", function_amount as "size_t"] -> *mut std::ffi::c_void as "void*" {
                auto swapchainHook = new VMTHook(hook_object, function_amount);

                return swapchainHook;
            });

            VmtHook { object }
        }
    }

    pub fn hook(&mut self, replacement: *mut std::ffi::c_void, index: isize) {
        unsafe {
            let hook_object = self.object;

            cpp!([hook_object as "VMTHook*", replacement as "void*", index as "size_t"] {
                hook_object->Hook(replacement, index);
            });
        };
    }

    pub fn enable(&mut self) {
        unsafe {
            let hook_object = self.object;

            cpp!([hook_object as "VMTHook*"] {
                hook_object->Enable();
            });
        };
    }

    pub fn get_original(&mut self, index: isize) -> *mut std::ffi::c_void {
        unsafe {
            let hook_object = self.object;

            cpp!([hook_object as "VMTHook*", index as "size_t"] -> *mut std::ffi::c_void as "void*" {
                return hook_object->GetOriginal<void*>(index);
            })
        }
    }
}

pub fn init() {
    unsafe {
        let mut device: *mut std::ffi::c_void = std::ptr::null_mut();
        (**POINTERS.swapchain).GetDevice(
            &winapi::um::d3d11::ID3D11Device::uuidof(),
            (&mut device) as *mut *mut std::ffi::c_void,
        );

        RENDERER_DATA.device = Some(device as *mut winapi::um::d3d11::ID3D11Device);

        let mut context: *mut std::ffi::c_void = std::ptr::null_mut();
        (*RENDERER_DATA.device.unwrap()).GetImmediateContext(
            (&mut context) as *mut *mut std::ffi::c_void
                as *mut *mut winapi::um::d3d11::ID3D11DeviceContext,
        );
        RENDERER_DATA.context = Some(context as *mut winapi::um::d3d11::ID3D11DeviceContext);
    }
}

pub fn register_present_callback(callback: D3DCallback) {
    unsafe {
        RENDERER_DATA.present_callbacks.push(callback);
    };
}

// pub fn register_pre_resize_callback(callback: D3DCallback) {
//     unsafe {
//         RENDERER_DATA.pre_resize_callbacks.push(callback);
//     };
// }

// pub fn register_post_resize_callback(callback: D3DCallback) {
//     unsafe {
//         RENDERER_DATA.post_resize_callbacks.push(callback);
//     };
// }

pub(crate) fn present(swapchain: *mut winapi::shared::dxgi::IDXGISwapChain) {
    unsafe {
        let mut device: *mut std::ffi::c_void = std::ptr::null_mut();
        (*swapchain).GetDevice(
            &winapi::um::d3d11::ID3D11Device::uuidof(),
            (&mut device) as *mut *mut std::ffi::c_void,
        );

        let device = device as *mut winapi::um::d3d11::ID3D11Device;

        let mut context: *mut std::ffi::c_void = std::ptr::null_mut();
        (*device).GetImmediateContext(
            (&mut context) as *mut *mut std::ffi::c_void
                as *mut *mut winapi::um::d3d11::ID3D11DeviceContext,
        );

        let context = context as *mut winapi::um::d3d11::ID3D11DeviceContext;

        for callback in &RENDERER_DATA.present_callbacks {
            callback(device, context);
        }
    }
}

// pub(crate) fn pre_resize() {
//     unsafe {
//         for callback in &RENDERER_DATA.pre_resize_callbacks {
//             callback();
//         }
//     }
// }

// pub(crate) fn post_resize() {
//     unsafe {
//         for callback in &RENDERER_DATA.post_resize_callbacks {
//             callback();
//         }
//     }
// }

cpp! {{

#include <cstdint>
#include <memory>

class VMTHook {
    public:
        explicit VMTHook(void* object, std::size_t numFuncs) noexcept :
            m_Object(reinterpret_cast<std::uintptr_t**>(object)),
            m_NumFuncs(numFuncs),
            m_Original(*m_Object),
            m_New(std::make_unique<std::uintptr_t[]>(m_NumFuncs + 1)) {
            std::copy_n(m_Original - 1, m_NumFuncs + 1, m_New.get());
        }

        ~VMTHook() noexcept = default;

        void Enable() noexcept {
            *m_Object = &m_New[1];
        }

        void Disable() noexcept {
            *m_Object = m_Original;
        }

        void Hook(void* replacement, std::size_t index) noexcept {
            m_New[index + 1] = reinterpret_cast<std::uintptr_t>(replacement);
        }

        void Unhook(std::size_t index) noexcept {
            m_New[index + 1] = m_Original[index];
        }

        template <typename T>
        T GetOriginal(std::size_t index) noexcept {
            return reinterpret_cast<T>(m_Original[index]);
        }

    private:
        std::uintptr_t** m_Object;
        std::size_t m_NumFuncs;

        std::uintptr_t* m_Original;
        std::unique_ptr<std::uintptr_t[]> m_New;
    };

}}
