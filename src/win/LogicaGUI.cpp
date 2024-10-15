#include <pluginterfaces/base/funknown.h>
#if SMTG_OS_WINDOWS
#include <windows.h>
#include "LogicaGUI.h"
#include "../LogicaEditor.h"

extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

static LRESULT CALLBACK LogicaWndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam) {
  // https://learn.microsoft.com/ja-jp/windows/win32/api/winuser/nf-winuser-getwindowlongptrw
  auto const gui = reinterpret_cast<logica::win::LogicaGUI*>(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
  if (gui) {
    return gui->WndProc(hwnd, msg, wParam, lParam);
  }
  return ::DefWindowProcW(hwnd, msg, wParam, lParam);
}

namespace logica::win {

static constexpr float clearColorWithAlpha[4] = {0.1f, 0.1f, 0.1f, 1.00f };

LogicaGUI::LogicaGUI(HWND windowHandle, LogicaEditor* editor)
:windowHandle_(windowHandle)
,editor_(editor)
{
}

/**************************************************************************************************
 * Win32 Window
 **************************************************************************************************/
void LogicaGUI::createWindowProc() {
  originalWindowFunc_ = reinterpret_cast<WNDPROC>(GetWindowLongPtrW(windowHandle_, GWLP_WNDPROC));
  originalWindowUserData_ = GetWindowLongPtrW(windowHandle_, GWLP_USERDATA);
  ::SetWindowLongPtrW(windowHandle_, GWLP_WNDPROC, reinterpret_cast<LONG_PTR>(LogicaWndProc));
  ::SetWindowLongPtrW(windowHandle_, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(this));
}

LRESULT WINAPI LogicaGUI::WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam) {
  if (useImGuiContext()) {
    LRESULT imguiResult = ImGui_ImplWin32_WndProcHandler(hWnd, msg, wParam, lParam);
    if (imguiResult != 0) {
      return imguiResult;
    }
    editor_->render();
  }

  switch(msg) {
    case WM_SIZE:
      if (wParam != SIZE_MINIMIZED) {
        UINT width = (UINT)LOWORD(lParam);
        UINT height = (UINT)HIWORD(lParam);
        resize(width, height);
      }
      return 0;
    case WM_SYSCOMMAND:
      // Disable ALT application menu
      if ((wParam & 0xfff0) == SC_KEYMENU) {
        return 0;
      }
      break;
    case WM_DESTROY:
      ::PostQuitMessage(0);
      return 0;
    default:
      if (originalWindowFunc_) {
        ::SetWindowLongPtrW(windowHandle_, GWLP_WNDPROC, reinterpret_cast<LONG_PTR>(originalWindowFunc_));
        ::SetWindowLongPtrW(windowHandle_, GWLP_USERDATA, originalWindowUserData_);
        LRESULT r = CallWindowProcW(originalWindowFunc_, windowHandle_, msg, wParam, lParam);
        ::SetWindowLongPtrW(windowHandle_, GWLP_WNDPROC, reinterpret_cast<LONG_PTR>(LogicaWndProc));
        ::SetWindowLongPtrW(windowHandle_, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(this));
        if (r != 0) {
          return r;
        }
      }
      break;
  }
  return ::DefWindowProcW(hWnd, msg, wParam, lParam);
}

/**************************************************************************************************
 * DirectX12
 **************************************************************************************************/

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
  if (D3D12CreateDevice(nullptr, featureLevel, IID_PPV_ARGS(&d3dDevice_)) != S_OK) {
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
    if (d3dDevice_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&d3dRtvDescHeap_)) != S_OK) {
      return false;
    }

    SIZE_T rtvDescriptorSize = d3dDevice_->GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);
    D3D12_CPU_DESCRIPTOR_HANDLE rtvHandle = d3dRtvDescHeap_->GetCPUDescriptorHandleForHeapStart();
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
    if (d3dDevice_->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&d3dSrvDescHeap_)) != S_OK) {
      return false;
    }
  }

  {
    D3D12_COMMAND_QUEUE_DESC desc = {};
    desc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    desc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
    desc.NodeMask = 1;
    if (d3dDevice_->CreateCommandQueue(&desc, IID_PPV_ARGS(&d3dCommandQueue_)) != S_OK) {
      return false;
    }
  }

  for (UINT i = 0; i < NUM_FRAMES_IN_FLIGHT; i++)
    if (d3dDevice_->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID_PPV_ARGS(&frameContext_[i].CommandAllocator)) != S_OK) {
      return false;
    }

  if (d3dDevice_->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, frameContext_[0].CommandAllocator, nullptr, IID_PPV_ARGS(&d3dCommandList_)) != S_OK ||
      d3dCommandList_->Close() != S_OK
  ) {
    return false;
  }

  if (d3dDevice_->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&fence_)) != S_OK) {
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
    if (dxgiFactory->CreateSwapChainForHwnd(d3dCommandQueue_, windowHandle_, &sd, nullptr, nullptr, &swapChain1) != S_OK) {
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
    d3dDevice_->CreateRenderTargetView(pBackBuffer, nullptr, mainRenderTargetDescriptor_[i]);
    mainRenderTargetResource_[i] = pBackBuffer;
  }
}

void LogicaGUI::cleanupDeviceD3D() {
  cleanupRenderTarget();
  HRESULT result;
  if (pSwapChain_) {
    result = pSwapChain_->SetFullscreenState(false, nullptr);
    assert(SUCCEEDED(result) && "Failed to make swap chain to be not full screen.");
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
  if (d3dCommandQueue_) {
    d3dCommandQueue_->Release();
    d3dCommandQueue_ = nullptr;
  }
  if (d3dCommandList_) {
    d3dCommandList_->Release();
    d3dCommandList_ = nullptr;
  }
  if (d3dRtvDescHeap_) {
    d3dRtvDescHeap_->Release();
    d3dRtvDescHeap_ = nullptr;
  }
  if (d3dSrvDescHeap_) {
    d3dSrvDescHeap_->Release();
    d3dSrvDescHeap_ = nullptr;
  }
  if (fence_) {
    fence_->Release();
    fence_ = nullptr;
  }
  if (fenceEvent_) {
    CloseHandle(fenceEvent_);
    fenceEvent_ = nullptr;
  }
  if (d3dDevice_) {
    d3dDevice_->Release();
    d3dDevice_ = nullptr;
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
  waitForLastSubmittedFrame();

  for (UINT i = 0; i < NUM_BACK_BUFFERS; i++) {
    if (mainRenderTargetResource_[i]) {
      mainRenderTargetResource_[i]->Release();
      mainRenderTargetResource_[i] = nullptr;
    }
  }
}

void LogicaGUI::waitForLastSubmittedFrame() {
  FrameContext& frameCtx = frameContext_[frameIndex_ % NUM_FRAMES_IN_FLIGHT];

  UINT64 fenceValue = frameCtx.FenceValue;
  if (fenceValue == 0)
    return; // No fence was signaled

  frameCtx.FenceValue = 0;
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

/**************************************************************************************************
 * ImGUI
 **************************************************************************************************/

void LogicaGUI::createImGui() {
  if (imguiContext_) {
    return;
  }
  IMGUI_CHECKVERSION();
  imguiContext_ = ImGui::CreateContext();
  // Set configs.
  useImGuiContext();
  ImGuiIO& io = ImGui::GetIO();
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableSetMousePos;  // Enable Mouse pos control
  io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
  // Do not save to external file!
  io.IniFilename = nullptr;
  // Do not log to external file!
  io.LogFilename = nullptr;
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
  d3dCommandList_->Reset(frameCtx->CommandAllocator, nullptr);
  d3dCommandList_->ResourceBarrier(1, &barrier);

  // Render Dear ImGui graphics
  d3dCommandList_->ClearRenderTargetView(mainRenderTargetDescriptor_[backBufferIdx], clearColorWithAlpha, 0, nullptr);
  d3dCommandList_->OMSetRenderTargets(1, &mainRenderTargetDescriptor_[backBufferIdx], FALSE, nullptr);
  d3dCommandList_->SetDescriptorHeaps(1, &d3dSrvDescHeap_);
  ImGui_ImplDX12_RenderDrawData(ImGui::GetDrawData(), d3dCommandList_);
  barrier.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
  barrier.Transition.StateAfter  = D3D12_RESOURCE_STATE_PRESENT;
  d3dCommandList_->ResourceBarrier(1, &barrier);
  d3dCommandList_->Close();

  d3dCommandQueue_->ExecuteCommandLists(1, (ID3D12CommandList* const*)&d3dCommandList_);

  // Present
  HRESULT hr = pSwapChain_->Present(1, 0);   // Present with vsync
  //HRESULT hr = g_pSwapChain->Present(0, 0); // Present without vsync
  swapChainOccluded_ = (hr == DXGI_STATUS_OCCLUDED);

  UINT64 fenceValue = fenceLastSignaledValue_ + 1;
  d3dCommandQueue_->Signal(fence_, fenceValue);
  fenceLastSignaledValue_ = fenceValue;
  frameCtx->FenceValue = fenceValue;
}

bool LogicaGUI::prepare() {
  if (!windowHandle_) {
    return false;
  }
  createWindowProc();
  // https://github.com/ocornut/imgui/tree/master/examples/example_win32_directx12
  if(!createDeviceD3D()) {
    cleanupDeviceD3D();
    return false;
  }
  // Setup Dear ImGui context
  createImGui();
  // Setup Dear ImGui style
  ImGui::StyleColorsDark();
  //ImGui::StyleColorsLight();
  // Setup Platform/Renderer backends
  ImGui_ImplWin32_Init(windowHandle_);
  ImGui_ImplDX12_Init(d3dDevice_, LogicaGUI::NUM_FRAMES_IN_FLIGHT,
                      DXGI_FORMAT_R8G8B8A8_UNORM, d3dSrvDescHeap_,
                      d3dSrvDescHeap_->GetCPUDescriptorHandleForHeapStart(),
                      d3dSrvDescHeap_->GetGPUDescriptorHandleForHeapStart());
  // Show the window
  ShowWindow(windowHandle_, SW_SHOWDEFAULT);
  UpdateWindow(windowHandle_);
  return true;
}

void LogicaGUI::cleanup() {
  // Destroy ImGUI
  cleanupImGui();
  // DX12 cleanup
  cleanupDeviceD3D();
  // window cleanup
  if (windowHandle_) {
    //::DestroyWindow(windowHandle_);
    windowHandle_ = nullptr;
  }
}

bool LogicaGUI::resize(size_t width, size_t height) {
  if (d3dDevice_ == nullptr || pSwapChain_ == nullptr) {
    return false;
  }
  waitForLastSubmittedFrame();

  cleanupRenderTarget();
  HRESULT result = pSwapChain_->ResizeBuffers(
      0,
      (UINT)width,
      (UINT)height,
      DXGI_FORMAT_R8G8B8A8_UNORM,
      DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT
  );
  assert(SUCCEEDED(result) && "Failed to resize swap chain.");
  createRenderTarget();
  return true;
}

}

#endif
