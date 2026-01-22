cmake_minimum_required(VERSION 3.20)

project(EkgProto
  VERSION 1.0
  DESCRIPTION "Generates C++ protobuf files from EKG schema"
  LANGUAGES CXX
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Protobuf REQUIRED)

set(PROTO_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../proto_schema")
set(PROTO_FILE "${PROTO_DIR}/data.proto")

set(Protobuf_IMPORT_DIRS "${PROTO_DIR}")

# TODO: Need to use nanopb somehow here instead of generate cpp
protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS "${PROTO_FILE}")

add_library(ekg_proto STATIC ${PROTO_SRCS} ${PROTO_HDRS})
target_link_libraries(ekg_proto PUBLIC protobuf::libprotobuf)

target_include_directories(ekg_proto PUBLIC
  $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}>
  $<BUILD_INTERFACE:${PROTO_DIR}>
)

add_executable(ekg_app main.cpp)
target_link_libraries(ekg_app PRIVATE ekg_proto)
