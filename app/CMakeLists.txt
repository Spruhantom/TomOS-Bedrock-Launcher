cmake_minimum_required(VERSION 3.21)

project(TomOSBedrockLauncher VERSION 0.1.0 LANGUAGES CXX)

find_package(Qt6 6.4 REQUIRED COMPONENTS Quick QuickControls2)

qt_standard_project_setup(REQUIRES 6.4)

qt_add_executable(tomos-bedrock-ui
    src/main.cpp
)

qt_add_qml_module(tomos-bedrock-ui
    URI TomOS.BedrockLauncher
    VERSION 1.0
    QML_FILES
        qml/Main.qml
        qml/BackendCard.qml
)

target_link_libraries(tomos-bedrock-ui
    PRIVATE
        Qt6::Quick
        Qt6::QuickControls2
)
