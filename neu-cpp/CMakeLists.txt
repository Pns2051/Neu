cmake_minimum_required(VERSION 3.16)

project(neu-cpp VERSION 1.0.0 LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick Network Multimedia Sql NetworkAuth)

add_executable(neu-cpp
    src/main.cpp
    src/core/MusicPlugin.cpp
    src/plugins/LocalPlugin.cpp
    src/plugins/YouTubePlugin.cpp
    ui/main.qml
    res/resources.qrc
)

target_include_directories(neu-cpp PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/src 
    ${CMAKE_CURRENT_SOURCE_DIR}/src/core
    ${CMAKE_CURRENT_SOURCE_DIR}/src/plugins
    ${CMAKE_CURRENT_SOURCE_DIR}/third_party
)

target_link_libraries(neu-cpp PRIVATE 
    Qt6::Core 
    Qt6::Gui 
    Qt6::Qml 
    Qt6::Quick 
    Qt6::Network
    Qt6::Multimedia
    Qt6::Sql
    Qt6::NetworkAuth
)

# Linux Specific Optimizations
if(LINUX)
    add_definitions(-DNEU_LINUX_OPTIMIZED)
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(ALSA REQUIRED alsa)
    pkg_check_modules(PULSE REQUIRED libpulse)
    target_link_libraries(neu-cpp PRIVATE ${ALSA_LIBRARIES} ${PULSE_LIBRARIES})
endif()

# CPack Configuration for .deb and .rpm
set(CPACK_GENERATOR "DEB" "RPM" "TGZ")
set(CPACK_PACKAGE_NAME "neu-music")
set(CPACK_PACKAGE_VENDOR "Pns2051")
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "High Performance C++/Qt Music Client")
set(CPACK_DEBIAN_PACKAGE_DEPENDS "libqt6core6, libqt6gui6, libqt6qml6, libqt6quick6, libqt6network6, libqt6multimedia6")
set(CPACK_RPM_PACKAGE_REQUIRES "qt6-qtbase, qt6-qtdeclarative, qt6-qtmultimedia")

include(CPack)
