cmake_minimum_required(VERSION 3.29.0)

option(SMTG_ENABLE_VST3_PLUGIN_EXAMPLES "Enable VST 3 Plug-in Examples" OFF)
option(SMTG_ENABLE_VST3_HOSTING_EXAMPLES "Enable VST 3 Hosting Examples" OFF)

set(CMAKE_OSX_DEPLOYMENT_TARGET 10.13 CACHE STRING "")
# FIXME: To Avoid create_symlink
set(SMTG_CREATE_PLUGIN_LINK OFF)

set(vst3sdk_SOURCE_DIR "./external/vst3sdk")
if(NOT vst3sdk_SOURCE_DIR)
  message(FATAL_ERROR "Path to VST3 SDK is empty!")
endif()

project(Logica
    # This is your plug-in version number. Change it here only.
    # Version number symbols usable in C++ can be found in
    #   src/version.h and ${PROJECT_BINARY_DIR}/projectversion.h.
    VERSION 1.0.0.0
    DESCRIPTION "Logica VST 3 Plug-in"
)

set(SMTG_VSTGUI_ROOT "${vst3sdk_SOURCE_DIR}")

add_subdirectory(${vst3sdk_SOURCE_DIR} ${PROJECT_BINARY_DIR}/vst3sdk)
smtg_enable_vst3_sdk()

set(IMGUI_SRCS
    external/imgui/backends/imgui_impl_dx12.h
    external/imgui/backends/imgui_impl_dx12.cpp
    external/imgui/backends/imgui_impl_win32.h
    external/imgui/backends/imgui_impl_win32.cpp
    external/imgui/imgui.h
    external/imgui/imgui.cpp
    external/imgui/imgui_draw.cpp
    external/imgui/imgui_tables.cpp
    external/imgui/imgui_widgets.cpp
)

set(LOGICA_SRCS
    src/version.h
    src/LogicaCIDs.h
    src/LogicaProcessor.h
    src/LogicaProcessor.cpp
    src/LogicaController.h
    src/LogicaController.cpp
    src/LogicaPluginView.h
    src/LogicaPluginView.cpp
    src/LogicaEntry.cpp
    src/LogicaUI.h
    src/Util.h
    src/Util.cpp
    src/win/ContentsFrame.h
    src/win/ContentsFrame.cpp
    src/win/Util.h
    src/win/Util.cpp
)

smtg_add_vst3plugin(Logica
    ${IMGUI_SRCS}
    ${LOGICA_SRCS}
)
target_include_directories(Logica PRIVATE external/imgui)
target_link_libraries(Logica PRIVATE "d3d12.lib" "dxgi.lib" "d3dcompiler.lib")

#- VSTGUI Wanted ----
if(SMTG_ENABLE_VSTGUI_SUPPORT)
  target_sources(Logica
      PRIVATE
      res/myplugineditor.uidesc
  )
  target_link_libraries(Logica
      PRIVATE
      vstgui_support
  )
  smtg_target_add_plugin_resources(Logica
      RESOURCES
      "res/myplugineditor.uidesc"
  )
endif(SMTG_ENABLE_VSTGUI_SUPPORT)
# -------------------

smtg_target_add_plugin_snapshots (Logica
    RESOURCES
    res/BFF7043CE3DA507DB73DDEDD2F77C358_snapshot.png
    res/BFF7043CE3DA507DB73DDEDD2F77C358_snapshot_2.0x.png
)

target_link_libraries(Logica
    PRIVATE
    sdk
)

smtg_target_configure_version_file(Logica)

if(SMTG_MAC)
  smtg_target_set_bundle(Logica
      BUNDLE_IDENTIFIER org.7io.logica
      COMPANY_NAME "ledyba"
  )
  smtg_target_set_debug_executable(Logica
      "/Applications/VST3PluginTestHost.app"
      "--pluginfolder;$(BUILT_PRODUCTS_DIR)"
  )
elseif(SMTG_WIN)
  target_sources(Logica PRIVATE
      res/win32resource.rc
  )
  if(MSVC)
    set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT Logica)

    smtg_target_set_debug_executable(Logica
        "$(ProgramW6432)/Steinberg/VST3PluginTestHost/VST3PluginTestHost.exe"
        "--pluginfolder \"$(OutDir)/\""
    )
  endif()
endif(SMTG_MAC)

# -------------------------------------------------------------------------------------------------
# Standalone
# -------------------------------------------------------------------------------------------------

add_executable(
    StandaloneTest
    ${IMGUI_SRCS}
    ${LOGICA_SRCS}
    src/win/main.cpp
)
target_include_directories(StandaloneTest PRIVATE external/imgui)
target_include_directories(StandaloneTest PRIVATE ${CMAKE_CURRENT_BINARY_DIR})

target_link_libraries(StandaloneTest
    PRIVATE
    base
    sdk
    sdk_common
    sdk_hosting
    pluginterfaces
    vstgui
    vstgui_support
    "d3d12.lib" "dxgi.lib" "d3dcompiler.lib"
)
