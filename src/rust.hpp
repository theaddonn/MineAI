#pragma once
#include <cstdint>
#include <string>

// Inlcudes for things used by the Rust code
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "Secur32.lib")
#pragma comment(lib, "Crypt32.lib")
#pragma comment(lib, "Bcrypt.lib")
#pragma comment(lib, "Ncrypt.lib")
#pragma comment(lib, "ntdll.lib")

enum MineAIInitDevice {
    CudaIfAvailable,
    CPU,
    CUDA,
    Vulkan,
};

struct MineAIInitOptions
{
    MineAIInitDevice mDevice;
    int64_t mMinLength;
    int64_t mMaxLength;
};

struct MineAIObject
{
    void* mModel;
};

struct MineAIInitResult
{
    std::string* mError;
    MineAIObject* mObject;
};

struct MineAIGenerationResult
{
    std::string* mError;
    std::string* mText;
};

