//------------------------------------------------------------------------
// Copyright(c) 2024 My Plug-in Company.
//------------------------------------------------------------------------

#include "LogicaController.h"
#include "LogicaCIDs.h"
#include "LogicaPluginView.h"
#include "imgui.h"

using namespace Steinberg;

namespace logica {

//------------------------------------------------------------------------
// LogicaController Implementation
//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::initialize(FUnknown* context) {
  // Here the Plug-in will be instantiated

  //---do not forget to call parent ------
  tresult result = EditControllerEx1::initialize(context);
  if (result != kResultOk) {
    return result;
  }

  // Here you could register some parameters

  return result;
}

//------------------------------------------------------------------------
tresult PLUGIN_API LogicaController::terminate() {
  // Here the Plug-in will be de-instantiated, last possibility to remove some memory!

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

void LogicaController::render() {
  ImGui::Begin("Hello, world!", &open_, ImGuiWindowFlags_MenuBar);                          // Create a window called "Hello, world!" and append into it.

  ImGui::Text("This is some useful text.");               // Display some text (you can use a format strings too)

  ImGui::SliderFloat("float", &f_, 0.0f, 1.0f);            // Edit 1 float using a slider from 0.0f to 1.0f
  ImGui::ColorEdit3("clear color", (float*)&clearColor_); // Edit 3 floats representing a color

  if (ImGui::Button("Button"))                            // Buttons return true when clicked (most widgets return true when edited/activated)
    counter_++;
  ImGui::SameLine();
  ImGui::Text("counter = %d", counter_);
  ImGuiIO& io = ImGui::GetIO();
  ImGui::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / io.Framerate, io.Framerate);
  ImGui::End();
}

//------------------------------------------------------------------------
} // namespace Logica
