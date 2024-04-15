add_executable(test)

target_link_libraries(
    test
    PUBLIC
    gtest
    gtest_main
)

target_sources(
    test
    PRIVATE
    8BitTest.cpp
    16BitTest.cpp
    32BitTest.cpp
    64BitTest.cpp
)

target_include_directories(
    test
    PUBLIC
    ../environment/
)
