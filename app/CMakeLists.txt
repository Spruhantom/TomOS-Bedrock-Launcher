cmake_minimum_required(VERSION 3.21)

project(TomOSBedrockLauncher VERSION 0.1.0 LANGUAGES CXX)

find_package(Qt6 6.4 REQUIRED COMPONENTS Quick QuickControls2)
find_program(CARGO_EXECUTABLE cargo REQUIRED)

qt_standard_project_setup(REQUIRES 6.4)

set(RUST_WORKSPACE ${CMAKE_CURRENT_SOURCE_DIR}/..)
set(RUST_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR}/cargo-target)
set(RUST_SERVICE ${RUST_TARGET_DIR}/release/tomos-bedrock-service)

file(GLOB_RECURSE RUST_SOURCES CONFIGURE_DEPENDS
    ${RUST_WORKSPACE}/crates/*.rs
    ${RUST_WORKSPACE}/crates/*/Cargo.toml
)
list(APPEND RUST_SOURCES ${RUST_WORKSPACE}/Cargo.toml)

add_custom_command(
    OUTPUT ${RUST_SERVICE}
    COMMAND ${CMAKE_COMMAND} -E env
        CARGO_TARGET_DIR=${RUST_TARGET_DIR}
        ${CARGO_EXECUTABLE} build --release
        --manifest-path ${RUST_WORKSPACE}/Cargo.toml
        --bin tomos-bedrock-service
    DEPENDS ${RUST_SOURCES}
    WORKING_DIRECTORY ${RUST_WORKSPACE}
    COMMENT "Building TomOS Bedrock Rust dashboard service"
    VERBATIM
)
add_custom_target(tomos-bedrock-service-build ALL DEPENDS ${RUST_SERVICE})

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

add_dependencies(tomos-bedrock-ui tomos-bedrock-service-build)

target_compile_definitions(tomos-bedrock-ui PRIVATE
    TOMOS_SERVICE_PATH="${RUST_SERVICE}"
)

target_link_libraries(tomos-bedrock-ui
    PRIVATE
        Qt6::Quick
        Qt6::QuickControls2
)
