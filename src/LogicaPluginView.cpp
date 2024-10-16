#include "LogicaPluginView.h"
#include "LogicaController.h"
#include "Util.h"

#if SMTG_OS_WINDOWS
#include <windows.h>
#include <winuser.h>
#include <imgui.h>
#include "win/ContentsFrame.h"
#endif

namespace logica {

using Steinberg::kResultTrue;
using Steinberg::kResultFalse;
using Steinberg::kInvalidArgument;
using Steinberg::kNotInitialized;

LogicaPluginView::ViewRect LogicaPluginView::DEFAULT_SIZE = makeViewRect(800, 600);

LogicaPluginView::LogicaPluginView(LogicaController* controller)
:controller_(controller)
{
}

LogicaPluginView::tresult LogicaPluginView::isPlatformTypeSupported(LogicaPluginView::FIDString type) {
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

LogicaPluginView::tresult LogicaPluginView::attached(void* parent, LogicaPluginView::FIDString type) {
  if (isPlatformTypeSupported(type) != kResultTrue) {
    return kResultFalse;
  }
#if SMTG_OS_WINDOWS
  if (contentsFrame_) {
    contentsFrame_->cleanup();
    contentsFrame_.reset();
  }
  contentsFrame_ = std::make_unique<ContentsFrame>(reinterpret_cast<HWND>(parent), DEFAULT_SIZE, controller_);
  if(!contentsFrame_->prepare()) {
    contentsFrame_->cleanup();
    contentsFrame_.reset();
    return kResultFalse;
  }
#endif
  return kResultTrue;
}

LogicaPluginView::tresult LogicaPluginView::removed() {
  if (!contentsFrame_) {
    return kResultFalse;
  }
  // Wait last frame.
  contentsFrame_->waitForLastSubmittedFrame();
  // ImGui cleanup & DX12 cleanup & windows cleanup
  contentsFrame_->cleanup();
  contentsFrame_.reset();
  return kResultTrue;
}

LogicaPluginView::tresult LogicaPluginView::onWheel(float distance) {
  return 0;
}

LogicaPluginView::tresult
LogicaPluginView::onKeyDown(LogicaPluginView::char16 key, LogicaPluginView::int16 keyCode, LogicaPluginView::int16 modifiers) {
  return 0;
}

LogicaPluginView::tresult
LogicaPluginView::onKeyUp(LogicaPluginView::char16 key, LogicaPluginView::int16 keyCode, LogicaPluginView::int16 modifiers) {
  return 0;
}

LogicaPluginView::tresult LogicaPluginView::getSize(LogicaPluginView::ViewRect *size) {
  if (size == nullptr) {
    return kInvalidArgument;
  }
  if (!contentsFrame_) {
    return kResultFalse;
  }
  *size = contentsFrame_->size();
  return kResultTrue;
}

LogicaPluginView::tresult LogicaPluginView::onSize(LogicaPluginView::ViewRect* newSize) {
  if (contentsFrame_ && newSize) {
    BOOL r = MoveWindow(
        contentsFrame_->windowHandle(),
        newSize->left,
        newSize->top,
        newSize->getWidth(),
        newSize->getHeight(),
        true
    );
    return r ? kResultTrue : kResultFalse;
  }
  return kResultFalse;
}

LogicaPluginView::tresult LogicaPluginView::onFocus(LogicaPluginView::TBool state) {
  return 0;
}

LogicaPluginView::tresult LogicaPluginView::setFrame(LogicaPluginView::IPlugFrame* frame) {
  pluginFrame_ = frame;
  return kResultTrue;
}

LogicaPluginView::tresult LogicaPluginView::canResize() {
  return kResultTrue;
}

LogicaPluginView::tresult LogicaPluginView::checkSizeConstraint(LogicaPluginView::ViewRect *rect) {
  return kResultTrue;
}

} // logica
