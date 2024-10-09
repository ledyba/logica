//
// Created by kaede on 2024/10/03.
//

#include "LogicaGUI.h"

namespace logica {
namespace win {

LogicaGUI::LogicaGUI(HWND hwnd)
:hwnd_(hwnd)
{

}

bool LogicaGUI::createDeviceD3D() {
  // Setup swap chain
  DXGI_SWAP_CHAIN_DESC1 sd;
  {
    ZeroMemory(&sd, sizeof(sd));
    sd.BufferCount = NUM_BACK_BUFFERS;
    sd.Width = 0;
    sd.Height = 0;
    sd.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    sd.Flags = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT;
    sd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    sd.SampleDesc.Count = 1;
    sd.SampleDesc.Quality = 0;
    sd.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
    sd.AlphaMode = DXGI_ALPHA_MODE_UNSPECIFIED;
    sd.Scaling = DXGI_SCALING_STRETCH;
    sd.Stereo = FALSE;
  }

  // [DEBUG] Enable debug interface
#ifdef DX12_ENABLE_DEBUG_LAYER
  ID3D12Debug* pdx12Debug = nullptr;
    if (SUCCEEDED(D3D12GetDebugInterface(IID_PPV_ARGS(&pdx12Debug))))
        pdx12Debug->EnableDebugLayer();
#endif

  // Create device
  D3D_FEATURE_LEVEL featureLevel = D3D_FEATURE_LEVEL_11_0;
  if (D3D12CreateDevice(nullptr, featureLevel, IID_PPV_ARGS(&pd3dDevice_)) != S_OK) {
    return false;
  }

  // [DEBUG] Setup debug interface to break on any warnings/errors
#ifdef DX12_ENABLE_DEBUG_LAYER
  if (pdx12Debug != nullptr)
    {
        ID3D12InfoQueue* pInfoQueue = nullptr;
        g_pd3dDevice->QueryInterface(IID_PPV_ARGS(&pInfoQueue));
        pInfoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_ERROR, true);
        pInfoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_CORRUPTION, true);
        pInfoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_WARNING, true);
        pInfoQueue->Release();
        pdx12Debug->Release();
    }
#endif

  {
    D3D12_DESCRIPTOR_HEAP_DESC desc = {};
    desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    desc.NumDescriptors = NUM_BACK_BUFFERS;
    desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
    desc.NodeMask = 1;
    if (pd3dDevice_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&pd3dRtvDescHeap_)) != S_OK) {
      return false;
    }

    SIZE_T rtvDescriptorSize = pd3dDevice_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
    D3D12_CPU_DESCRIPTOR_HANDLE rtvHandle = pd3dRtvDescHeap_->GetCPUDescriptorHandleForHeapStart();
    for (UINT i = 0; i < NUM_BACK_BUFFERS; i++) {
      mainRenderTargetDescriptor_[i] = rtvHandle;
      rtvHandle.ptr += rtvDescriptorSize;
    }
  }

  {
    D3D12_DESCRIPTOR_HEAP_DESC desc = {};
    desc.Type = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
    desc.NumDescriptors = 1;
    desc.Flags = D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
    if (pd3dDevice_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&pd3dSrvDescHeap_)) != S_OK) {
      return false;
    }
  }

  {
    D3D12_COMMAND_QUEUE_DESC desc = {};
    desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
    desc.NodeMask = 1;
    if (pd3dDevice_->CreateCommandQueue(&desc, IID_PPV_ARGS(&pd3dCommandQueue_)) != S_OK) {
      return false;
    }
  }

  for (UINT i = 0; i < NUM_FRAMES_IN_FLIGHT; i++)
    if (pd3dDevice_->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID_PPV_ARGS(&frameContext_[i].CommandAllocator)) != S_OK) {
      return false;
    }

  if (pd3dDevice_->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, frameContext_[0].CommandAllocator, nullptr, IID_PPV_ARGS(&pd3dCommandList_)) != S_OK ||
      pd3dCommandList_->Close() != S_OK
  ) {
    return false;
  }

  if (pd3dDevice_->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&fence_)) != S_OK) {
    return false;
  }

  fenceEvent_ = CreateEvent(nullptr, FALSE, FALSE, nullptr);
  if (fenceEvent_ == nullptr) {
    return false;
  }

  {
    IDXGIFactory4* dxgiFactory = nullptr;
    IDXGISwapChain1* swapChain1 = nullptr;
    if (CreateDXGIFactory1(IID_PPV_ARGS(&dxgiFactory)) != S_OK)
      return false;
    if (dxgiFactory->CreateSwapChainForHwnd(pd3dCommandQueue_, hwnd_, &sd, nullptr, nullptr, &swapChain1) != S_OK) {
      return false;
    }
    if (swapChain1->QueryInterface(IID_PPV_ARGS(&pSwapChain_)) != S_OK) {
      return false;
    }
    swapChain1->Release();
    dxgiFactory->Release();
    pSwapChain_->SetMaximumFrameLatency(NUM_BACK_BUFFERS);
    hSwapChainWaitableObject_ = pSwapChain_->GetFrameLatencyWaitableObject();
  }

  createRenderTarget();
  return true;
}

void LogicaGUI::createRenderTarget() {
  for (UINT i = 0; i < NUM_BACK_BUFFERS; i++) {
    ID3D12Resource* pBackBuffer = nullptr;
    pSwapChain_->GetBuffer(i, IID_PPV_ARGS(&pBackBuffer));
    pd3dDevice_->CreateRenderTargetView(pBackBuffer, nullptr, mainRenderTargetDescriptor_[i]);
    mainRenderTargetResource_[i] = pBackBuffer;
  }
}

void LogicaGUI::cleanupDeviceD3D() {
  cleanupRenderTarget();
  if (pSwapChain_) {
    pSwapChain_->SetFullscreenState(false, nullptr);
    pSwapChain_->Release();
    pSwapChain_ = nullptr;
  }
  if (hSwapChainWaitableObject_ != nullptr) {
    CloseHandle(hSwapChainWaitableObject_);
  }
  for (UINT i = 0; i < NUM_FRAMES_IN_FLIGHT; i++) {
    if (frameContext_[i].CommandAllocator) {
      frameContext_[i].CommandAllocator->Release();
      frameContext_[i].CommandAllocator = nullptr;
    }
  }
  if (pd3dCommandQueue_) {
    pd3dCommandQueue_->Release();
    pd3dCommandQueue_ = nullptr;
  }
  if (pd3dCommandList_) {
    pd3dCommandList_->Release();
    pd3dCommandList_ = nullptr;
  }
  if (pd3dRtvDescHeap_) {
    pd3dRtvDescHeap_->Release();
    pd3dRtvDescHeap_ = nullptr;
  }
  if (pd3dSrvDescHeap_) {
    pd3dSrvDescHeap_->Release();
    pd3dSrvDescHeap_ = nullptr;
  }
  if (fence_) {
    fence_->Release();
    fence_ = nullptr;
  }
  if (fenceEvent_) {
    CloseHandle(fenceEvent_);
    fenceEvent_ = nullptr;
  }
  if (pd3dDevice_) {
    pd3dDevice_->Release();
    pd3dDevice_ = nullptr;
  }

#ifdef DX12_ENABLE_DEBUG_LAYER
  IDXGIDebug1* pDebug = nullptr;
    if (SUCCEEDED(DXGIGetDebugInterface1(0, IID_PPV_ARGS(&pDebug))))
    {
        pDebug->ReportLiveObjects(DXGI_DEBUG_ALL, DXGI_DEBUG_RLO_SUMMARY);
        pDebug->Release();
    }
#endif
}

void LogicaGUI::cleanupRenderTarget() {
  for (UINT i = 0; i < NUM_BACK_BUFFERS; i++) {
    ID3D12Resource* pBackBuffer = nullptr;
    pSwapChain_->GetBuffer(i, IID_PPV_ARGS(&pBackBuffer));
    pd3dDevice_->CreateRenderTargetView(pBackBuffer, nullptr, mainRenderTargetDescriptor_[i]);
    mainRenderTargetResource_[i] = pBackBuffer;
  }
}

void LogicaGUI::waitForLastSubmittedFrame() {
  FrameContext* frameCtx = &frameContext_[frameIndex_ % NUM_FRAMES_IN_FLIGHT];

  UINT64 fenceValue = frameCtx->FenceValue;
  if (fenceValue == 0)
    return; // No fence was signaled

  frameCtx->FenceValue = 0;
  if (fence_->GetCompletedValue() >= fenceValue) {
    return;
  }

  fence_->SetEventOnCompletion(fenceValue, fenceEvent_);
  WaitForSingleObject(fenceEvent_, INFINITE);
}

LogicaGUI::FrameContext *LogicaGUI::waitForNextFrameResources() {
  UINT nextFrameIndex = frameIndex_ + 1;
  frameIndex_ = nextFrameIndex;

  HANDLE waitableObjects[] = { hSwapChainWaitableObject_, nullptr };
  DWORD numWaitableObjects = 1;

  FrameContext* frameCtx = &frameContext_[nextFrameIndex % NUM_FRAMES_IN_FLIGHT];
  UINT64 fenceValue = frameCtx->FenceValue;
  if (fenceValue != 0) // means no fence was signaled
  {
    frameCtx->FenceValue = 0;
    fence_->SetEventOnCompletion(fenceValue, fenceEvent_);
    waitableObjects[1] = fenceEvent_;
    numWaitableObjects = 2;
  }

  WaitForMultipleObjects(numWaitableObjects, waitableObjects, TRUE, INFINITE);

  return frameCtx;
}

void LogicaGUI::createImGui() {
  if (imguiContext_) {
    return;
  }
  IMGUI_CHECKVERSION();
  imguiContext_ = ImGui::CreateContext();
  useImGuiContext();
  ImGuiIO& io = ImGui::GetIO(); (void)io;
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableSetMousePos;  // Enable Mouse pos control
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
}

void LogicaGUI::cleanupImGui() {
  if (imguiContext_) {
    useImGuiContext();
    ImGui_ImplDX12_Shutdown();
    ImGui_ImplWin32_Shutdown();
    ImGui::DestroyContext(imguiContext_);
    imguiContext_ = nullptr;
  }
}

bool LogicaGUI::useImGuiContext() {
  if (imguiContext_) {
    ImGui::SetCurrentContext(imguiContext_);
    return true;
  }
  return false;
}

void LogicaGUI::renderFinish() {
  FrameContext* frameCtx = waitForNextFrameResources();
  UINT backBufferIdx = pSwapChain_->GetCurrentBackBufferIndex();
  frameCtx->CommandAllocator->Reset();

  D3D12_RESOURCE_BARRIER barrier = {};
  barrier.Type                   = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
  barrier.Flags                  = D3D12_RESOURCE_BARRIER_FLAG_NONE;
  barrier.Transition.pResource   = mainRenderTargetResource_[backBufferIdx];
  barrier.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
  barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_PRESENT;
  barrier.Transition.StateAfter  = D3D12_RESOURCE_STATE_RENDER_TARGET;
  pd3dCommandList_->Reset(frameCtx->CommandAllocator, nullptr);
  pd3dCommandList_->ResourceBarrier(1, &barrier);

  // Render Dear ImGui graphics
  const float clear_color_with_alpha[4] = { 0.1f, 0.1f, 0.1f, 1.00f };
  pd3dCommandList_->ClearRenderTargetView(mainRenderTargetDescriptor_[backBufferIdx], clear_color_with_alpha, 0, nullptr);
  pd3dCommandList_->OMSetRenderTargets(1, &mainRenderTargetDescriptor_[backBufferIdx], FALSE, nullptr);
  pd3dCommandList_->SetDescriptorHeaps(1, &pd3dSrvDescHeap_);
  ImGui_ImplDX12_RenderDrawData(ImGui::GetDrawData(), pd3dCommandList_);
  barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
  barrier.Transition.StateAfter  = D3D12_RESOURCE_STATE_PRESENT;
  pd3dCommandList_->ResourceBarrier(1, &barrier);
  pd3dCommandList_->Close();

  pd3dCommandQueue_->ExecuteCommandLists(1, (ID3D12CommandList* const*)&pd3dCommandList_);

  // Present
  HRESULT hr = pSwapChain_->Present(1, 0);   // Present with vsync
  //HRESULT hr = g_pSwapChain->Present(0, 0); // Present without vsync
  swapChainOccluded_ = (hr == DXGI_STATUS_OCCLUDED);

  UINT64 fenceValue = fenceLastSignaledValue_ + 1;
  pd3dCommandQueue_->Signal(fence_, fenceValue);
  fenceLastSignaledValue_ = fenceValue;
  frameCtx->FenceValue = fenceValue;
}

bool LogicaGUI::prepare() {
  // https://github.com/ocornut/imgui/tree/master/examples/example_win32_directx12
  if(!createDeviceD3D()) {
    cleanupDeviceD3D();
    return false;
  }
  // Setup Dear ImGui context
  createImGui();
  return true;
}

void LogicaGUI::cleanup() {
  // Destroy ImGUI
  cleanupImGui();
  // DX12 cleanup
  cleanupDeviceD3D();
  // window cleanup
  if (hwnd_) {
    DestroyWindow(hwnd_);
    hwnd_ = nullptr;
  }
}

}}
