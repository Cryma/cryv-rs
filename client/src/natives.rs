use std::{ffi::c_void, sync::Mutex};

use crate::crossmap::CROSSMAP;
use cpp::cpp;
use detour::static_detour;
use log::error;
use once_cell::sync::OnceCell;
use winapi::{
    shared::minwindef::LPVOID,
    um::{
        timeapi::timeGetTime, winbase::ConvertThreadToFiber, winbase::CreateFiber,
        winbase::SwitchToFiber, winbase::LPFIBER_START_ROUTINE,
    },
};

const GET_FRAME_COUNT_NATIVE: u64 = 0x812595A0644CE1DE;

static_detour! {
    static GetFrameCount: fn(*mut c_void) -> *mut c_void;
}

#[derive(Copy, Clone)]
struct FiberWrapper {
    pointer: *mut c_void,
}

unsafe impl Send for FiberWrapper {}

static MAIN_FIBER: OnceCell<Mutex<FiberWrapper>> = OnceCell::new();
static SCRIPT_FIBER: OnceCell<Mutex<FiberWrapper>> = OnceCell::new();
static mut WAKE_AT: u32 = 0; // TODO: Find better solution for this

fn handler(hash: u64) -> Option<*mut c_void> {
    if hash == 0 {
        return None;
    }

    let registration_table = unsafe { crate::hook::POINTERS.registration_table };

    let value: *mut c_void = unsafe {
        cpp!([hash as "uint64_t", registration_table as "NativeRegistrationNew**"] -> *mut c_void as "void*" {
            auto *table = registration_table[hash & 0xFF];

            for (; table; table = table->getNextRegistration()) {
                for (uint32_t i = 0; i < table->getNumEntries(); i++) {
                    if (hash == table->getHash(i)) {
                        return (void*) table->handlers[i];
                    }
                }
            }

            return (void*) nullptr;
        })
    };

    Some(value)
}

fn get_native_handler(hash: u64) -> Option<*mut c_void> {
    // TODO: Implement native handler caching

    match handler(hash) {
        Some(value) => Some(value),
        None => None,
    }
}

fn map_native(in_native: u64) -> Option<u64> {
    match CROSSMAP.get(&in_native) {
        Some(value) => Some(*value),
        None => None,
    }
}

pub fn hook_get_frame_count() {
    let native = match map_native(GET_FRAME_COUNT_NATIVE) {
        Some(value) => value,
        None => {
            error!(
                "Could not find translation for native: {}",
                GET_FRAME_COUNT_NATIVE
            );

            return;
        }
    };

    let address = match get_native_handler(native) {
        Some(value) => value,
        None => {
            error!("Could not find native handler for native: {}", native);

            return;
        }
    };

    if let Err(error) =
        unsafe { GetFrameCount.initialize(std::mem::transmute(address), get_frame_count) }
    {
        error!("Error initializing GetFrameCount hook: {}", error);

        return;
    }

    if let Err(error) = unsafe { GetFrameCount.enable() } {
        error!("Error enabling GetFrameCount hook: {}", error);

        return;
    }
}

fn on_tick() {
    MAIN_FIBER.get_or_init(|| {
        Mutex::new(FiberWrapper {
            pointer: unsafe { ConvertThreadToFiber(std::ptr::null_mut()) },
        })
    });

    if unsafe { timeGetTime() < WAKE_AT } {
        return;
    }

    let mut has_initialized = false;
    let script_fiber = SCRIPT_FIBER.get_or_init(|| {
        has_initialized = true;

        // Create fiber for script function
        let func: LPFIBER_START_ROUTINE = Some(script_function);

        Mutex::new(FiberWrapper {
            pointer: unsafe { CreateFiber(0, func, std::ptr::null_mut()) },
        })
    });

    if has_initialized {
        return;
    }

    let script_fiber_value = *script_fiber.lock().unwrap();

    unsafe { SwitchToFiber(script_fiber_value.pointer) };
}

unsafe extern "system" fn script_function(_: LPVOID) {
    loop {
        script_wait(0);
    }
}

fn script_wait(time: u32) {
    let mut wake_time = unsafe { timeGetTime() } + time;

    if wake_time == unsafe { WAKE_AT } {
        wake_time += 1;
    }

    unsafe {
        WAKE_AT = wake_time;
    }

    let main_fiber = MAIN_FIBER.get();

    if let Some(value) = main_fiber {
        let main_fiber_value = *value.lock().unwrap();

        unsafe { SwitchToFiber(main_fiber_value.pointer) };
    }
}

fn get_frame_count(context: *mut c_void) -> *mut c_void {
    on_tick();

    context
}

// TODO: Implement this in Rust
cpp! {{

#include <unordered_map>
#include <Windows.h>
#include <cstdint>

class ScrNativeCallContext
{
protected:

    void* _return;
    uint32_t _argCount;
    void* _args;
    uint32_t _dataCount;
    alignas(uintptr_t)uint8_t _vectorSpace[192];

public:
    template<typename T>
    T getArgument(int idx)
    {
        intptr_t * arguments = static_cast<intptr_t*>(_args);
        return *static_cast<T*>(&arguments[idx]);
    }

    template<typename T>
    void setResult(int idx, T value)
    {
        intptr_t * returnValues = static_cast<intptr_t*>(_return);
        *static_cast<T*>(&returnValues[idx]) = value;
    }

    int getArgumentCount() const
    {
        return _argCount;
    }

    template<typename T>
    T getResult(int idx)
    {
        intptr_t * returnValues = static_cast<intptr_t*>(_return);
        return *static_cast<T*>(&returnValues[idx]);
    }

    static void(*SetVectorResults)(ScrNativeCallContext*);
};

class NativeContext : public ScrNativeCallContext
{
private:
    enum
    {
        MaxNativeParams = 16,
        ArgSize = 8,
    };

    uint8_t _tempStack[MaxNativeParams * ArgSize];

public:
    NativeContext()
    {
        _args = &_tempStack;
        _return = &_tempStack;

        _argCount = 0;
        _dataCount = 0;
    }

    template <typename T>
    void push(T value)
    {
        if ( sizeof( T ) > ArgSize )
        {
            throw "Argument has an invalid size";
        }

        if ( sizeof( T ) < ArgSize )
        {
            *reinterpret_cast<uintptr_t*>( _tempStack + ArgSize * _argCount ) = 0;
        }

        *reinterpret_cast<T*>( _tempStack + ArgSize * _argCount ) = value;
        _argCount++;
    }

    void reverse()
    {
        uintptr_t tempValues[MaxNativeParams];
        const auto args = static_cast<uintptr_t*>(_args);

        for (uint32_t i = 0; i < _argCount; i++)
        {
            const int target = _argCount - i - 1;
            tempValues[target] = args[i];
        }

        memcpy(_tempStack, tempValues, sizeof _tempStack);
    }

    template <typename T>
    T getResult()
    {
        return *reinterpret_cast<T*>(_tempStack);
    }

};

struct Pass
{
    template<typename ...T> Pass( T... ) {}
};

class NativeManagerContext : public NativeContext
{
public:

    NativeManagerContext() : NativeContext() {}

    void reset()
    {
        _argCount = 0;
        _dataCount = 0;
    }

    void* getResultPointer() const
    {
        return _return;
    }

};

typedef void(__cdecl *NativeHandler)(ScrNativeCallContext *context);

struct NativeRegistrationNew {
    uint64_t nextRegistration1;
    uint64_t nextRegistration2;
    NativeHandler handlers[7];
    uint32_t numEntries1;
    uint32_t numEntries2;
    uint64_t hashes;

    NativeRegistrationNew* getNextRegistration() {
        uintptr_t result;
        auto v5 = reinterpret_cast<uintptr_t>(&nextRegistration1);
        auto v12 = 2i64;
        const auto v13 = v5 ^ nextRegistration2;
        const auto v14 = reinterpret_cast<char *>(&result) - v5;
        do {
            *reinterpret_cast<DWORD*>(&v14[v5]) = (DWORD) v13 ^ *reinterpret_cast<DWORD*>(v5);
            v5 += 4i64;
            --v12;
        } while (v12);

        return reinterpret_cast<NativeRegistrationNew*>(result);
    }

    uint32_t getNumEntries() {
        return (uint32_t) reinterpret_cast<uintptr_t>(&numEntries1) ^ numEntries1 ^ numEntries2;
    }

    uint64_t getHash(uint32_t index) {

        auto naddr = 16 * index + reinterpret_cast<uintptr_t>(&nextRegistration1) + 0x54;
        auto v8 = 2i64;
        uint64_t nResult;
        auto v11 = (char *) &nResult - naddr;
        auto v10 = naddr ^ *(DWORD*) (naddr + 8);
        do {
            *(DWORD *) &v11[naddr] = (DWORD) v10 ^ *(DWORD*) (naddr);
            naddr += 4i64;
            --v8;
        } while (v8);

        return nResult;
    }
};


enum eThreadState {
    ThreadStateIdle = 0x0,
    ThreadStateRunning = 0x1,
    ThreadStateKilled = 0x2,
    ThreadState3 = 0x3,
    ThreadState4 = 0x4,
};

struct scrThreadContext {
    int ThreadID;
    int ScriptHash;
    eThreadState State;
    int _IP;
    int FrameSP;
    int _SPP;
    float TimerA;
    float TimerB;
    int TimerC;
    int _mUnk1;
    int _mUnk2;
    int _f2C;
    int _f30;
    int _f34;
    int _f38;
    int _f3C;
    int _f40;
    int _f44;
    int _f48;
    int _f4C;
    int _f50;
    int pad1;
    int pad2;
    int pad3;
    int _set1;
    int pad[17];
};

struct scrThread {
    void *vTable;
    scrThreadContext m_ctx;
    void *m_pStack;
    void *pad;
    void *pad2;
    const char *m_pszExitMessage;
};

struct ScriptThread : scrThread {
    const char Name[64];
    void *m_pScriptHandler;
    const char gta_pad2[40];
    const char flag1;
    const char m_networkFlag;
    bool bool1;
    bool bool2;
    bool bool3;
    bool bool4;
    bool bool5;
    bool bool6;
    bool bool7;
    bool bool8;
    bool bool9;
    bool bool10;
    bool bool11;
    bool bool12;
    const char gta_pad3[10];
};

}}
