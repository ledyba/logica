# pragma once
#include "public.sdk/source/vst/vstguieditor.h"

namespace logica {

class LogicaEditor: public Steinberg::Vst::VSTGUIEditor {
public:
  LogicaEditor(Steinberg::Vst::EditController* controller, Steinberg::ViewRect * size);
  ~LogicaEditor() override = default;

  bool PLUGIN_API open(void* parent, VSTGUI::PlatformType const& platformType) SMTG_OVERRIDE;
  void PLUGIN_API close() SMTG_OVERRIDE;
  Steinberg::tresult PLUGIN_API onSize(Steinberg::ViewRect* newSize) SMTG_OVERRIDE;

  DELEGATE_REFCOUNT(VSTGUIEditor)
};

class LogicaEditorView : public VSTGUI::COpenGLView {
public:
  explicit LogicaEditorView (VSTGUI::CRect const& size);
  void drawOpenGL(const VSTGUI::CRect& updateRect) override;
};

} // logica
