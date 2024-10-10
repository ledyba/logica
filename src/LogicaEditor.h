#pragma once

#include <memory>
#include <pluginterfaces/base/fplatform.h>

#include <pluginterfaces/gui/iplugview.h>
#include <pluginterfaces/base/funknown.h>
#include <pluginterfaces/vst/vsttypes.h>
#include <base/source/fobject.h>

namespace logica {

class LogicaController;

#if SMTG_OS_WINDOWS
namespace win {
class LogicaGUI;
}
using LogicaGUI = ::logica::win::LogicaGUI;
#endif

class LogicaEditor: public Steinberg::FObject, public Steinberg::IPlugView {
public:
  explicit LogicaEditor(LogicaController* controller);
  ~LogicaEditor() override = default;
public:
  using FIDString = Steinberg::FIDString;
  using char16 = Steinberg::char16;
  using int16 = Steinberg::int16;
  using TBool = Steinberg::TBool;
  using ViewRect = Steinberg::ViewRect;
  using IPlugFrame = Steinberg::IPlugFrame;
  using tresult = Steinberg::tresult;

  tresult PLUGIN_API isPlatformTypeSupported(FIDString type) SMTG_OVERRIDE;
  tresult PLUGIN_API attached(void* parent, FIDString type) SMTG_OVERRIDE;
  tresult PLUGIN_API removed() SMTG_OVERRIDE;
  tresult PLUGIN_API onWheel(float distance) SMTG_OVERRIDE;
  tresult PLUGIN_API onKeyDown(char16 key, int16 keyCode, int16 modifiers) SMTG_OVERRIDE;
  tresult PLUGIN_API onKeyUp(char16 key, int16 keyCode, int16 modifiers) SMTG_OVERRIDE;
  tresult PLUGIN_API getSize(ViewRect* size) SMTG_OVERRIDE;
  tresult PLUGIN_API onSize(ViewRect* newSize) SMTG_OVERRIDE;
  tresult PLUGIN_API onFocus(TBool state) SMTG_OVERRIDE;
  tresult PLUGIN_API setFrame(IPlugFrame* frame) SMTG_OVERRIDE;
  tresult PLUGIN_API canResize() SMTG_OVERRIDE;
  tresult PLUGIN_API checkSizeConstraint(ViewRect* rect) SMTG_OVERRIDE;

  OBJ_METHODS (LogicaEditor, FObject)
  DEFINE_INTERFACES
    DEF_INTERFACE (IPlugView)
  END_DEFINE_INTERFACES (FObject)
  REFCOUNT_METHODS (FObject)
public:
  void render();
private:
  LogicaController* controller_;
  ViewRect size_;
  Steinberg::IPtr<IPlugFrame> frame_;
  std::unique_ptr<LogicaGUI> gui_;
};

} // logica
