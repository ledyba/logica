#include "LogicaEditor.h"
#include "vstgui/lib/copenglview.h"

namespace logica {

LogicaEditor::LogicaEditor(Steinberg::Vst::EditController* const controller, Steinberg::ViewRect* const size)
:VSTGUIEditor(controller, size)
{
  setRect(*size);
}

bool LogicaEditor::open(void *parent, VSTGUI::PlatformType const& platformType) {
  if (frame) {
    return false;
  }
  auto rect = getRect();
  VSTGUI::CRect size(rect.left, rect.top, rect.right, rect.bottom);
  frame = new VSTGUI::CFrame(size, this);
  frame->addView(new LogicaEditorView(size));
  return frame->open(parent, platformType);
}

void LogicaEditor::close() {
  if (frame != nullptr) {
    frame->forget();
    frame = nullptr;
  }
}

// https://forums.steinberg.net/t/what-should-we-be-doing-in-our-editor-onsize-method/792100
Steinberg::tresult LogicaEditor::onSize(Steinberg::ViewRect* newSize) {
  return VSTGUIEditor::onSize(newSize);
}

LogicaEditorView::LogicaEditorView(VSTGUI::CRect const& size)
:COpenGLView(size)
{

}

void LogicaEditorView::drawOpenGL(VSTGUI::CRect const& updateRect)
{
}

} // logica
