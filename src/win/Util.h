#pragma once

#include <pluginterfaces/base/funknown.h>
#include <pluginterfaces/gui/iplugview.h>

Steinberg::ViewRect makeViewRect(int width, int height);

#if SMTG_OS_WINDOWS
#include <Windows.h>
bool isParentLayered(HWND parent);
#endif
