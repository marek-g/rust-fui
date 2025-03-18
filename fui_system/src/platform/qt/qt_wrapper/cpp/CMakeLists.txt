cmake_minimum_required(VERSION 3.16)

project(qt_wrapper VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Core Widgets OpenGL)
if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    find_package(KF6WindowSystem)
endif ()
qt_standard_project_setup()

qt_add_library(qt_wrapper STATIC
    qaction.cpp
    qapplication.cpp
    qbytearray.cpp
    qfiledialog.cpp
    qicon.cpp qmenu.cpp
    qopenglcontext.cpp
    qpixmap.cpp
    qslot.cpp
    qslot_cpp.cpp
    qstring.cpp
    qstringlist.cpp
    qsurfaceformat.cpp
    qsystemtrayicon.cpp
    qwindow.cpp
    qwindow_ext.cpp
)

target_include_directories(qt_wrapper PRIVATE ${FFI_INCLUDE_DIR})

target_link_libraries(qt_wrapper PRIVATE
    Qt6::Core
    Qt6::Widgets
    Qt6::OpenGL
if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    KF6::WindowSystem
endif ()
)

install(TARGETS qt_wrapper DESTINATION lib)
