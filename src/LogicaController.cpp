//------------------------------------------------------------------------
// Copyright(c) 2024 My Plug-in Company.
//------------------------------------------------------------------------

#include <pluginterfaces/base/ibstream.h> // To use IBStream

#include "LogicaController.h"
#include "LogicaCIDs.h"
#include "LogicaPluginView.h"

using namespace Steinberg;

namespace logica {

//------------------------------------------------------------------------
// LogicaController Implementation
//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::initialize(FUnknown* context) {
  // Here the Plug-in will be instantiated

  //---do not forget to call parent ------
  tresult result = EditControllerEx1::initialize(context);
  if (result != kResultTrue) {
    return result;
  }

  // Here you could register some parameters
  {
    namespace node = ax::NodeEditor;
    node::Config config;
    config.SaveNodeSettings = nullptr;
    config.SaveSettings = nullptr;
    config.LoadSettings = nullptr;
    config.LoadNodeSettings = nullptr;
    // config.SettingsFile = nullptr;
    config.UserPointer = this;
    config.EnableSmoothZoom = true;
    config.CanvasSizeMode = node::CanvasSizeMode::CenterOnly;
    nodeEditorContext_ = node::CreateEditor(&config);
    if (!nodeEditorContext_) {
      return kInternalError;
    }
  }

  return kResultTrue;
}

//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::terminate() {
  // Here the Plug-in will be de-instantiated, last possibility to remove some memory!
  if (nodeEditorContext_) {
    namespace node = ax::NodeEditor;
    node::DestroyEditor(nodeEditorContext_);
    nodeEditorContext_ = nullptr;
  }

  //---do not forget to call parent ------
  return EditControllerEx1::terminate ();
}

//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::setComponentState(IBStream* state) {
  // Here you get the state of the component (Processor part)
  if (!state) {
    return kResultFalse;
  }

  return kResultOk;
}

//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::setState(IBStream* state) {
  // Here you get the state of the controller

  return kResultTrue;
}

//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::getState(IBStream* state) {
  // Here you are asked to deliver the state of the controller (if needed)
  // Note: the real state of your plug-in is saved in the processor

  return kResultTrue;
}

//------------------------------------------------------------------------
IPlugView* PLUGIN_API LogicaController::createView(FIDString name) {
  // Here the Host wants to open your editor (if you have one)
  if (!FIDStringsEqual(name, Vst::ViewType::kEditor)) {
    return nullptr;
  }
  return new LogicaPluginView(this);
}

//------------------------------------------------------------------------

void LogicaController::render() {
  namespace node = ax::NodeEditor;
  ImGuiIO& io = ImGui::GetIO();
  ImGuiViewport* viewport = ImGui::GetMainViewport();
  ImGui::SetNextWindowPos(viewport->WorkPos);
  ImGui::SetNextWindowSize(viewport->WorkSize);
  ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0f);
  ImGui::PushStyleVar(ImGuiStyleVar_WindowMinSize, ImVec2(0.f, 0.f));
  ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0.f, 0.f));

  ImGui::GetStyle().WindowRounding = 0.0f;
  ImGui::Begin("NodeEditor", nullptr, ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoResize);
  node::SetCurrentEditor(nodeEditorContext_);
  {
    node::Begin("My Editor", viewport->WorkSize);
    int uniqueId = 1;
    // Start drawing nodes.
    node::BeginNode(uniqueId++);
    ImGui::Text("Node A");
    node::BeginPin(uniqueId++, node::PinKind::Input);
    ImGui::Text("-> In");
    node::EndPin();
    ImGui::SameLine();
    node::BeginPin(uniqueId++, node::PinKind::Output);
    ImGui::Text("Out ->");
    node::EndPin();
    node::EndNode();
    node::End();
  }
  node::SetCurrentEditor(nullptr);
  ImGui::End();
  ImGui::PopStyleVar(3);
}

} // namespace Logica
