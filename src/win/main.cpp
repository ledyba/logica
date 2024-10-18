#include <pluginterfaces/base/funknown.h>

#if SMTG_OS_WINDOWS
#include <Windows.h>
#include <memory>
#include "ContentsFrame.h"
#include "../Util.h"
#include "../LogicaController.h"

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
    case WM_CLOSE:
      DestroyWindow(hWnd);
      return 0;
    default:
      break;
  }
  return ::DefWindowProcW(hWnd, msg, wParam, lParam);
}

int main(int argc, char** argv) {
  WNDCLASSEXW wc = {
      sizeof(wc),
      CS_CLASSDC | CS_HREDRAW,
      WndProc,
      0L,
      0L,
      GetModuleHandle(nullptr),
      nullptr,
      LoadCursor(nullptr, IDC_ARROW),
      nullptr,
      nullptr,
      L"LogicaParent",
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
  auto* ui = new logica::LogicaController();
  ui->initialize(nullptr); // TODO: Error handling
  auto frame_ = std::make_unique<logica::win::ContentsFrame>(hwnd, logica::makeViewRect(1280, 800), ui);
  if (frame_->prepare()) {
    MSG msg = {};
    while (msg.message != WM_QUIT) {
      if (::PeekMessageW(&msg, nullptr, 0U, 0U, PM_REMOVE)) {
        ::TranslateMessage(&msg);
        ::DispatchMessageW(&msg);
      }
    }
  }
  frame_->cleanup();
  ui->terminate();
  ui->release();
  ::DestroyWindow(hwnd);
  ::UnregisterClassW(wc.lpszClassName, wc.hInstance);
  return 0;
}

#endif
