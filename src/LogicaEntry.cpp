//------------------------------------------------------------------------
// Copyright(c) 2024 My Plug-in Company.
//------------------------------------------------------------------------

#include <public.sdk/source/main/pluginfactory.h>

#include "LogicaProcessor.h"
#include "LogicaController.h"
#include "LogicaCIDs.h"
#include "version.h"

#define stringPluginName "Logica"

using namespace Steinberg::Vst;
using namespace logica;

//------------------------------------------------------------------------
//  VST Plug-in Entry
//------------------------------------------------------------------------

BEGIN_FACTORY_DEF (
    "ledyba",
    "https://7io.org",
    "mailto:psi@7io.org"
  )

  //---First Plug-in included in this factory-------
  // its kVstAudioEffectClass component
  DEF_CLASS2 (INLINE_UID_FROM_FUID(kLogicaProcessorUID),
        PClassInfo::kManyInstances,  // cardinality
        kVstAudioEffectClass,  // the component category (do not changed this)
        stringPluginName,    // here the Plug-in name (to be changed)
        Vst::kDistributable,  // means that component and controller could be distributed on different computers
        LogicaVST3Category, // Subcategory for this Plug-in (to be changed)
        FULL_VERSION_STR,    // Plug-in version (to be changed)
        kVstVersionString,    // the VST 3 SDK version (do not changed this, use always this define)
        LogicaProcessor::createInstance // function pointer called when this component should be instantiated
     )

  // its kVstComponentControllerClass component
  DEF_CLASS2 (INLINE_UID_FROM_FUID (kLogicaControllerUID),
        PClassInfo::kManyInstances, // cardinality
        kVstComponentControllerClass,// the Controller category (do not changed this)
        stringPluginName "Controller",  // controller name (could be the same than component name)
        0,            // not used here
        "",            // not used here
        FULL_VERSION_STR,    // Plug-in version (to be changed)
        kVstVersionString,    // the VST 3 SDK version (do not changed this, use always this define)
        LogicaController::createInstance // function pointer called when this component should be instantiated
      )

  //----for others Plug-ins contained in this factory, put like for the first Plug-in different DEF_CLASS2---

END_FACTORY
