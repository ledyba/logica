#include "LogicaEditor.h"
#include "LogicaGUI.h"
#if SMTG_OS_WINDOWS
#include <windows.h>
#include <imgui.h>
#endif
namespace logica {

using Steinberg::kResultTrue;
using Steinberg::kResultFalse;
using Steinberg::kInvalidArgument;

LogicaEditor::LogicaEditor(LogicaController* controller)
:controller_(controller)
,size_(0, 0, 640, 480)
,gui_(nullptr)
{
}

LogicaEditor::tresult LogicaEditor::isPlatformTypeSupported(LogicaEditor::FIDString type) {
#if SMTG_OS_WINDOWS
  using Steinberg::kPlatformTypeHWND;
  if (strcmp (type, kPlatformTypeHWND) == 0)
    return kResultTrue;
#elif SMTG_OS_MACOS
#if TARGET_OS_IPHONE
  using Steinberg::kPlatformTypeUIView;
  if (strcmp (type, kPlatformTypeUIView) == 0)
    return kResultTrue;
#else
  using Steinberg::kPlatformTypeNSView;
  if (strcmp (type, kPlatformTypeNSView) == 0)
    return kResultTrue;
#endif // TARGET_OS_IPHONE
#elif SMTG_OS_LINUX
  using Steinberg::kPlatformTypeX11EmbedWindowID;
  if (strcmp (type, kPlatformTypeX11EmbedWindowID) == 0)
    return kResultTrue;
#endif
  return kInvalidArgument;
}

LogicaEditor::tresult LogicaEditor::attached(void* parent, LogicaEditor::FIDString type) {
  if (isPlatformTypeSupported(type) != kResultTrue) {
    return kResultFalse;
  }
  if (frame_) {
    frame_->resizeView(this, &this->size_);
  }
#if SMTG_OS_WINDOWS
  HWND hwnd = reinterpret_cast<HWND>(parent);
  if (gui_) {
    gui_->cleanup();
    gui_.reset();
  }

  gui_ = std::make_unique<LogicaGUI>(hwnd);
  if(!gui_->prepare()) {
    gui_->cleanup();
    gui_.reset();
    return kResultFalse;
  }
  gui_->useImGuiContext();

  // Setup Dear ImGui style
  ImGui::StyleColorsDark();
  //ImGui::StyleColorsLight();
  // Show the window
  ShowWindow(hwnd, SW_SHOWDEFAULT);
  UpdateWindow(hwnd);
  {
    // Setup Platform/Renderer backends
    ImGui_ImplWin32_Init(hwnd);
    ImGui_ImplDX12_Init(gui_->d3d12Device(), LogicaGUI::NUM_FRAMES_IN_FLIGHT,
                        DXGI_FORMAT_R8G8B8A8_UNORM, gui_->d3dSrvDescHeap(),
                        gui_->d3dSrvDescHeap()->GetCPUDescriptorHandleForHeapStart(),
                        gui_->d3dSrvDescHeap()->GetGPUDescriptorHandleForHeapStart());
    // TODO: LOOP
    ImGui_ImplDX12_NewFrame();
    ImGui_ImplWin32_NewFrame();
    ImGui::NewFrame();
    {
      ImGui::Begin("Hello, world!");                          // Create a window called "Hello, world!" and append into it.

      ImGui::Text("This is some useful text.");               // Display some text (you can use a format strings too)

      ImGui::Button("Button");
      ImGui::SameLine();
      ImGui::Text("counter = 1");

      ImGui::End();
    }
    ImGui::Render();
    gui_->renderFinish();
  }
#endif
  return 0;
}

LogicaEditor::tresult LogicaEditor::removed() {
  if(gui_) {
    return kResultFalse;
  }
  // Wait last frame.
  gui_->waitForLastSubmittedFrame();

  // ImGui cleanup & DX12 cleanup & windows cleanup
  gui_->cleanup();
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::onWheel(float distance) {
  return 0;
}

LogicaEditor::tresult
LogicaEditor::onKeyDown(LogicaEditor::char16 key, LogicaEditor::int16 keyCode, LogicaEditor::int16 modifiers) {
  return 0;
}

LogicaEditor::tresult
LogicaEditor::onKeyUp(LogicaEditor::char16 key, LogicaEditor::int16 keyCode, LogicaEditor::int16 modifiers) {
  return 0;
}

LogicaEditor::tresult LogicaEditor::getSize(LogicaEditor::ViewRect *size) {
  if (size == nullptr) {
    return kInvalidArgument;
  }
  *size = size_;
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::onSize(LogicaEditor::ViewRect *newSize) {
  return 0;
}

LogicaEditor::tresult LogicaEditor::onFocus(LogicaEditor::TBool state) {
  return 0;
}

LogicaEditor::tresult LogicaEditor::setFrame(LogicaEditor::IPlugFrame* frame) {
  frame_ = frame;
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::canResize() {
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::checkSizeConstraint(LogicaEditor::ViewRect *rect) {
  return kResultTrue;
}

} // logica
