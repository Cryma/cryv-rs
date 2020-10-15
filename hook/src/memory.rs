use cpp::cpp;
use std::ffi::{c_void, CString};

pub fn get_pattern(pattern: String, add: i32) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char *", add as "uint32_t"] -> *mut c_void as "char*" {
            return Signature(pattern_raw_pointer).scan().add(add).as<char*>();
        })
    };

    data
}

pub fn get_pattern_sub(pattern: String, sub: i32) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char *", sub as "uint32_t"] -> *mut c_void as "char*" {
            return Signature(pattern_raw_pointer).scan().sub(sub).as<char*>();
        })
    };

    data
}

pub fn get_pattern_rip(pattern: String, add: i32) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char *", add as "uint32_t"] -> *mut c_void as "char*" {
            return Signature(pattern_raw_pointer).scan().add(add).rip().as<char*>();
        })
    };

    data
}

pub fn address_fill(address: *mut c_void, count: usize, element: u8) {
    for index in 0..count {
        unsafe {
            *((address as *mut u8).add(index)) = element;
        };
    }
}

pub fn get_pattern_in_memory_region(
    pattern: String,
    region: *mut c_void,
    size: i32,
) -> *mut c_void {
    let pattern_raw = CString::new(pattern).unwrap();
    let pattern_raw_pointer = pattern_raw.as_ptr();

    let data = unsafe {
        cpp!([pattern_raw_pointer as "const char*", region as "void*", size as "int"] -> *mut c_void as "void*" {
            MemoryRegion memory_region(MemoryHandle(region), size);
            return Signature(pattern_raw_pointer).scan(memory_region).as<void*>();
        })
    };

    data
}

cpp! {{

#include <cstdint>
#include <type_traits>
#include <Windows.h>
#include <vector>

class MemoryHandle
{
public:
    constexpr MemoryHandle(void* p = nullptr) : _ptr(p)
    {
    }

    explicit MemoryHandle(std::uintptr_t p) : _ptr(reinterpret_cast<void*>(p))
    {
    }

    template<typename T>
    constexpr std::enable_if_t<std::is_pointer_v<T>, T> as()
    {
        return static_cast<T>(_ptr);
    }

    template<typename T>
    constexpr std::enable_if_t<std::is_lvalue_reference_v<T>, T> as()
    {
        return *static_cast<std::add_pointer_t<std::remove_reference_t<T>>>(_ptr);
    }

    template<typename T>
    constexpr std::enable_if_t<std::is_same_v<T, std::uintptr_t>, T> as()
    {
        return reinterpret_cast<T>(_ptr);
    }

    template<typename T>
    constexpr MemoryHandle add(T offset)
    {
        return MemoryHandle(as<std::uintptr_t>() + offset);
    }

    template<typename T>
    constexpr MemoryHandle sub(T offset)
    {
        return MemoryHandle(as<std::uintptr_t>() - offset);
    }

    constexpr MemoryHandle rip()
    {
        if(_ptr == false)
        {
            return nullptr;
        }

        return add(as<std::int32_t&>()).add(4U);
    }

    constexpr explicit operator bool() noexcept
    {
        return _ptr;
    }

protected:
    void *_ptr;
};

class MemoryRegion
{
public:
    constexpr explicit MemoryRegion(MemoryHandle base, std::size_t size) : _base(base), _size(size)
    {
    }

    constexpr MemoryHandle base() const
    {
        return _base;
    }

    constexpr MemoryHandle end()
    {
        return _base.add(_size);
    }

    constexpr std::size_t size() const
    {
        return _size;
    }

    constexpr bool contains(MemoryHandle p)
    {
        if(p.as<std::uintptr_t>() < _base.as<std::uintptr_t>())
        {
            return false;
        }

        if(p.as<std::uintptr_t>() > _base.add(_size).as<std::uintptr_t>())
        {
            return false;
        }

        return true;
    }

protected:
    MemoryHandle _base;
    std::size_t _size;
};

class Module : public MemoryRegion
{
public:
    explicit Module(std::nullptr_t) : Module(static_cast<char*>(nullptr))
    {
    }

    explicit Module(const char *name) : Module(GetModuleHandleA(name))
    {
    }

    Module(HMODULE hmodule) : MemoryRegion(hmodule, 0)
    {
        const auto dosHeader = _base.as<IMAGE_DOS_HEADER*>();
        const auto ntHeader = _base.add(dosHeader->e_lfanew).as<IMAGE_NT_HEADERS64*>();
        _size = ntHeader->OptionalHeader.SizeOfImage;
    }

    IMAGE_DOS_HEADER *getDosHeader()
    {
        return _base.as<IMAGE_DOS_HEADER*>();
    }

    IMAGE_NT_HEADERS64 *getNtHeaders()
    {
        return _base.add(_base.as<IMAGE_DOS_HEADER*>()->e_lfanew).as<IMAGE_NT_HEADERS64*>();
    }

private:
    template<typename TReturn, typename TOffset>
    TReturn getRVA(TOffset rva)
    {
        return _base.add(rva).as<TReturn>();
    }
};

class Signature
{
public:
    struct Element
    {
        std::uint8_t _data{};
        bool _wildcard{};
    };

    explicit Signature(const char *pattern) {
        auto toUpper = [](char c) -> char {
            return c >= 'a' && c <= 'z' ? static_cast<char>(c + ('A' - 'a')) : static_cast<char>(c);
        };

        auto isHex = [&](char c) -> bool {
            switch (toUpper(c)) {
            case '0':
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
            case '6':
            case '7':
            case '8':
            case '9':
            case 'A':
            case 'B':
            case 'C':
            case 'D':
            case 'E':
            case 'F':
                return true;
            default:
                return false;

            }
        };

        do {
            if (*pattern == ' ') {
                continue;
            }

            if (*pattern == '?') {
                _elements.push_back(Element{ {}, true });
                continue;
            }

            if(*(pattern + 1) && isHex(*pattern) && isHex(*(pattern + 1)))
            {
                char str[3] = { *pattern, *(pattern + 1), '\0' };
                auto data = std::strtol(str, nullptr, 16);

                _elements.push_back(Element{ static_cast<std::uint8_t>(data), false });
            }
        } while (*(pattern++));
    }

    MemoryHandle scan(MemoryRegion region = Module(nullptr))
    {
        auto compareMemory = [](std::uint8_t *data, Element *element, std::size_t num) -> bool
        {
            for(std::size_t i = 0; i < num; ++i)
            {
                if(element[i]._wildcard == false)
                {
                    if(data[i] != element[i]._data)
                    {
                        return false;
                    }
                }
            }

            return true;
        };

        for(std::uintptr_t i = region.base().as<std::uintptr_t>(), end = region.end().as<std::uintptr_t>(); i != end; ++i)
        {
            if(compareMemory(reinterpret_cast<std::uint8_t*>(i), _elements.data(), _elements.size()))
            {
                return MemoryHandle(i);
            }
        }

        return {};
    }

    private:
        std::vector<Element> _elements;
};

}}
