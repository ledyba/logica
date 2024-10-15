#include "LogicaEditor.h"

#if SMTG_OS_WINDOWS
#include "win/LogicaGUI.h"
#include <windows.h>
#include <winuser.h>
#include <imgui.h>
#endif

namespace logica {

using Steinberg::kResultTrue;
using Steinberg::kResultFalse;
using Steinberg::kInvalidArgument;
using Steinberg::kNotInitialized;

LogicaEditor::LogicaEditor(LogicaController* controller)
:controller_(controller)
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
    frame_->resizeView(this, &LogicaGUI::DEFAULT_SIZE);
  }
#if SMTG_OS_WINDOWS
  if (gui_) {
    gui_->cleanup();
    gui_.reset();
  }
  gui_ = std::make_unique<LogicaGUI>(reinterpret_cast<HWND>(parent), this);
  if(!gui_->prepare()) {
    gui_->cleanup();
    return kResultFalse;
  }
  render();
#endif
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::removed() {
  if (!gui_) {
    return kResultFalse;
  }
  // Wait last frame.
  gui_->waitForLastSubmittedFrame();
  // ImGui cleanup & DX12 cleanup & windows cleanup
  gui_->cleanup();
  gui_.reset();
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
  if (!gui_) {
    return kResultFalse;
  }
  *size = gui_->size();
  return kResultTrue;
}

LogicaEditor::tresult LogicaEditor::onSize(LogicaEditor::ViewRect* newSize) {
  if (gui_ && newSize) {
    SetWindowPos(
        gui_->windowHandle(),
        nullptr,
        newSize->left,
        newSize->top,
        newSize->getWidth(),
        newSize->getHeight(),
        SWP_SHOWWINDOW
    );
  }
  if (!gui_ && gui_->resize(*newSize)) {
    return kResultTrue;
  }
  return kResultFalse;
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

void LogicaEditor::render() {
  ImGui_ImplDX12_NewFrame();
  ImGui_ImplWin32_NewFrame();
  ImGui::NewFrame();
  bool open = true;
  {
    // Create a window called "Hello, world!" and append into it.
    ImGui::Begin("Hello, world!", &open, ImGuiWindowFlags_AlwaysAutoResize);

    ImGui::Text("This is some useful text.");               // Display some text (you can use a format strings too)

    ImGui::Button("Button");
    ImGui::SameLine();
    ImGui::Text("counter = 1");

    ImGui::End();
  }
  ImGui::Render();
  gui_->renderFinish();
}

} // logica
