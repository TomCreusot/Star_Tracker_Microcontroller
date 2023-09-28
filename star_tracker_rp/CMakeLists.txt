# Min version cmake allowed.
cmake_minimum_required(VERSION 3.12)

# Ensure you have downloaded the sdk first.
# Try running setup.sh.
include($ENV{PICO_SDK_PATH}/external/pico_sdk_import.cmake)


project(star_tracker_rp C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

pico_sdk_init()

# Where executable source is.
add_executable(${PROJECT_NAME}
    main.c
)





# Link HAL stdlib
target_link_libraries(
    ${PROJECT_NAME} pico_stdlib
    ${CMAKE_SOURCE_DIR}/target/thumbv6m-none-eabi/release/libstar_tracker_rp.a
)



# Hex uf2 files
pico_add_extra_outputs(${PROJECT_NAME})

# USB DEBUGGING
# enable usb output, disable uart output
pico_enable_stdio_usb(${PROJECT_NAME} 1)
pico_enable_stdio_uart(${PROJECT_NAME} 0)
