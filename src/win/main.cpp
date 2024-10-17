#include <pluginterfaces/base/funknown.h>

#if SMTG_OS_WINDOWS
#include <Windows.h>

LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam) {
  switch (msg) {
    case WM_SYSCOMMAND:
      if ((wParam & 0xfff0) == SC_KEYMENU) {// Disable ALT application menu
        return 0;
      }
      break;
    case WM_DESTROY:
      ::PostQuitMessage(0);
      return 0;
    default:
      break;
  }
  return ::DefWindowProcW(hWnd, msg, wParam, lParam);
}

int main(int argc, char** argv) {
  WNDCLASSEXW wc = {
      sizeof(wc),
      CS_CLASSDC,
      WndProc,
      0L,
      0L,
      GetModuleHandle(nullptr),
      nullptr,
      nullptr,
      nullptr,
      nullptr,
      L"Logica",
      nullptr
  };
  ::RegisterClassExW(&wc);
  HWND hwnd = ::CreateWindowExW(
      0L,
      wc.lpszClassName,
      L"Logica GUI Test",
      WS_OVERLAPPEDWINDOW,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      1280,
      800,
      nullptr,
      nullptr,
      wc.hInstance,
      nullptr
  );
  ::ShowWindow(hwnd, SW_SHOWDEFAULT);
  ::UpdateWindow(hwnd);
  bool done = false;
  while (!done) {
    // Poll and handle messages (inputs, window resize, etc.)
    // See the WndProc() function below for our to dispatch events to the Win32 backend.
    MSG msg;
    while (::PeekMessage(&msg, nullptr, 0U, 0U, PM_REMOVE)) {
      ::TranslateMessage(&msg);
      ::DispatchMessage(&msg);
      if (msg.message == WM_QUIT) {
        done = true;
      }
    }
    if (done) {
      break;
    }
  }
  ::DestroyWindow(hwnd);
  ::UnregisterClassW(wc.lpszClassName, wc.hInstance);
  return 0;
}

#endif
