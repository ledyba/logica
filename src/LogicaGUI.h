#pragma once

#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx12.h>
#include <d3d12.h>
#include <dxgi1_4.h>
#include <tchar.h>

namespace logica {
namespace win {

class LogicaGUI {
public:
  explicit LogicaGUI(HWND hwnd);
private:
  struct FrameContext {
    ID3D12CommandAllocator *CommandAllocator;
    UINT64 FenceValue;
  };
public:
  int static constexpr NUM_FRAMES_IN_FLIGHT = 3;
  int static constexpr NUM_BACK_BUFFERS = 3;
private:
  HWND hwnd_ = nullptr;
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
  [[nodiscard]] HWND hwnd() const { return hwnd_; }
  [[nodiscard]] ID3D12Device* d3d12Device() const { return pd3dDevice_; }
  [[nodiscard]] ID3D12DescriptorHeap* d3dSrvDescHeap() const { return pd3dSrvDescHeap_; }
};

}}
