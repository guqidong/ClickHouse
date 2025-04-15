if (ENABLE_EXAMPLES)
    add_subdirectory(examples)
endif ()

if (ENABLE_FUZZING)
    add_subdirectory(fuzzers)
endif()

add_compile_options( -fbracket-depth=4096 )
# if increase to 4096 does not work, we'll try the next line
#add_compile_options(-Wno-stack-exhausted)
