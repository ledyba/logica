//------------------------------------------------------------------------
// Copyright(c) 2024 My Plug-in Company.
//------------------------------------------------------------------------

#pragma once

#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/vst/vsttypes.h"

namespace Logica {
//------------------------------------------------------------------------
static const Steinberg::FUID kLogicaProcessorUID(0xBFF7043C, 0xE3DA507D, 0xB73DDEDD, 0x2F77C358);
static const Steinberg::FUID kLogicaControllerUID(0xA1038D20, 0x45B450FC, 0xB5CE9F1C, 0x43448F81);

#define LogicaVST3Category "Instrument"

//------------------------------------------------------------------------
} // namespace Logica
