#pragma once

#include <pluginterfaces/base/funknown.h>

#if SMTG_OS_WINDOWS
#include <Windows.h>
bool isParentLayered(HWND parent);
#endif
