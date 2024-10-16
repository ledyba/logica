#include <pluginterfaces/base/funknown.h>
#if SMTG_OS_WINDOWS

#include "Util.h"

bool isParentLayered(HWND parent)
{
  WINDOWINFO info;
  info.cbSize = sizeof(info);
  while (parent) {
    if (GetWindowInfo (parent, &info)) {
      if (info.dwStyle & WS_CHILD)
        parent = GetParent (parent);
      else
        break;
    }
  }
  if (parent) {
    if (info.dwExStyle & WS_EX_LAYERED) {
      return true;
    }
  }
  return false;
}

#endif
