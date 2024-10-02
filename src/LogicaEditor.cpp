#include "LogicaEditor.h"
#if SMTG_OS_WINDOWS
#include <windef.h>
#endif
namespace logica {

using Steinberg::kResultTrue;
using Steinberg::kResultFalse;
using Steinberg::kInvalidArgument;

LogicaEditor::LogicaEditor(LogicaController* controller)
:controller_(controller)
,size_(0, 0, 640, 480)
{
}

LogicaEditor::tresult LogicaEditor::isPlatformTypeSupported(LogicaEditor::FIDString type) {
#if SMTG_OS_WINDOWS
  using Steinberg::kPlatformTypeHWND;
  if (strcmp (type, kPlatformTypeHWND) == 0)
    return kResultTrue;

#elif SMTG_OS_MACOS

  using Steinberg::kPlatformTypeUIView;
  #if TARGET_OS_IPHONE
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
#endif
  return 0;
}

LogicaEditor::tresult LogicaEditor::removed() {
  return 0;
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
