#pragma once

#if 0
#include <public.sdk/source/vst/vstguieditor.h>
#include <vstgui/lib/platform/win32/win32frame.h>
// See:
// https://www.utsbox.com/?p=1186
using VSTGUIEditor = Steinberg::Vst::VSTGUIEditor;
using CFrame = VSTGUI::CFrame;
using Win32Frame = VSTGUI::Win32Frame;
#endif

#include <tchar.h>
#include <pluginterfaces/base/fplatform.h>

#if SMTG_OS_WINDOWS

// ImGUI Includes
// https://github.com/ocornut/imgui/wiki/Getting-Started#example-if-you-are-using-raw-win32-api--directx12
#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx12.h>
#include <d3d12.h>
#include <dxgi1_4.h>

namespace logica::win {

class LogicaGUI {
public:
  explicit LogicaGUI(HWND windowHandle);
private:
  struct FrameContext {
    ID3D12CommandAllocator *CommandAllocator;
    UINT64 FenceValue;
  };
public:
  int static constexpr NUM_FRAMES_IN_FLIGHT = 3;
  int static constexpr NUM_BACK_BUFFERS = 3;
private:
  HWND windowHandle_ = nullptr;
  WNDPROC originalWindowFunc_ = nullptr;
  LONG_PTR originalWindowUserData_ = reinterpret_cast<LONG_PTR>(nullptr);
private:
  FrameContext frameContext_[NUM_FRAMES_IN_FLIGHT] = {};
  UINT frameIndex_ = 0;
private:
  ID3D12Device* pd3dDevice_ = nullptr;
  ID3D12DescriptorHeap* pd3dRtvDescHeap_ = nullptr;
  ID3D12DescriptorHeap* pd3dSrvDescHeap_ = nullptr;
  ID3D12CommandQueue* pd3dCommandQueue_ = nullptr;
  ID3D12GraphicsCommandList* pd3dCommandList_ = nullptr;
  ID3D12Fence* fence_ = nullptr;
  HANDLE fenceEvent_ = nullptr;
  UINT64 fenceLastSignaledValue_ = 0;
  IDXGISwapChain3* pSwapChain_ = nullptr;
  bool swapChainOccluded_ = false;
  HANDLE hSwapChainWaitableObject_ = nullptr;
  ID3D12Resource *mainRenderTargetResource_[NUM_BACK_BUFFERS] = {};
  D3D12_CPU_DESCRIPTOR_HANDLE mainRenderTargetDescriptor_[NUM_BACK_BUFFERS] = {};
private:
  ImGuiContext* imguiContext_ = nullptr;
private:
  void createWindowProc();
public:
  LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);
private:
  bool createDeviceD3D();
  void createRenderTarget();
  void createImGui();
  void cleanupDeviceD3D();
  void cleanupRenderTarget();
  void cleanupImGui();
public:
  void waitForLastSubmittedFrame();
  FrameContext* waitForNextFrameResources();
public:
  bool prepare();
  bool useImGuiContext();
  void renderFinish();
  void cleanup();
public:
  [[nodiscard]] HWND windowHandle() { return windowHandle_; }
  [[nodiscard]] ID3D12Device* d3d12Device() const { return pd3dDevice_; }
  [[nodiscard]] ID3D12DescriptorHeap* d3dSrvDescHeap() const { return pd3dSrvDescHeap_; }
};

}
#endif
