//------------------------------------------------------------------------
// Copyright(c) 2024 My Plug-in Company.
//------------------------------------------------------------------------

#pragma once

#include <public.sdk/source/vst/vsteditcontroller.h>
#include <imgui.h>
#include <imgui_node_editor.h>
#include "LogicaUI.h"

namespace logica {

//------------------------------------------------------------------------
//  LogicaController
//------------------------------------------------------------------------
class LogicaController : public Steinberg::Vst::EditControllerEx1, public LogicaUI
{
public:
//------------------------------------------------------------------------
  LogicaController() = default;
  ~LogicaController() SMTG_OVERRIDE = default;

  // Create function
  static Steinberg::FUnknown* createInstance(void* /*context*/) {
    return (Steinberg::Vst::IEditController*) new LogicaController;
  }

  //--- from IPluginBase -----------------------------------------------
  Steinberg::tresult PLUGIN_API initialize(Steinberg::FUnknown* context) SMTG_OVERRIDE;
  Steinberg::tresult PLUGIN_API terminate() SMTG_OVERRIDE;

  //--- from EditController --------------------------------------------
  Steinberg::tresult PLUGIN_API setComponentState(Steinberg::IBStream* state) SMTG_OVERRIDE;
  Steinberg::IPlugView* PLUGIN_API createView(Steinberg::FIDString name) SMTG_OVERRIDE;
  Steinberg::tresult PLUGIN_API setState(Steinberg::IBStream* state) SMTG_OVERRIDE;
  Steinberg::tresult PLUGIN_API getState(Steinberg::IBStream* state) SMTG_OVERRIDE;

   //---Interface---------
  DEFINE_INTERFACES
    // Here you can add more supported VST3 interfaces
    // DEF_INTERFACE (Vst::IXXX)
  END_DEFINE_INTERFACES(EditController)
  DELEGATE_REFCOUNT(EditController)

//------------------------------------------------------------------------
protected:
public:
  ax::NodeEditor::EditorContext* nodeEditorContext_;
  bool open_ = true;
  int counter_ = 0;
  float f_ = 0;
  float clearColor_[3] = {0, 0, 0};
  void render() override;
};

//------------------------------------------------------------------------
} // namespace Logica
