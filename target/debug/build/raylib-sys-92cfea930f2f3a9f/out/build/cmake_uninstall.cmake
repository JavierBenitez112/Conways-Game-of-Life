if(NOT EXISTS "C:/Users/javib/OneDrive/Escritorio/graficas/Game_of_life/target/debug/build/raylib-sys-92cfea930f2f3a9f/out/build/install_manifest.txt")
  message(FATAL_ERROR "Cannot find install manifest: C:/Users/javib/OneDrive/Escritorio/graficas/Game_of_life/target/debug/build/raylib-sys-92cfea930f2f3a9f/out/build/install_manifest.txt")
endif()

file(READ "C:/Users/javib/OneDrive/Escritorio/graficas/Game_of_life/target/debug/build/raylib-sys-92cfea930f2f3a9f/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach(file ${files})
  message(STATUS "Uninstalling $ENV{DESTDIR}${file}")
  if(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    exec_program(
      "C:/Program Files/CMake/bin/cmake.exe" ARGS "-E remove \"$ENV{DESTDIR}${file}\""
      OUTPUT_VARIABLE rm_out
      RETURN_VALUE rm_retval
      )
    if(NOT "${rm_retval}" STREQUAL 0)
      message(FATAL_ERROR "Problem when removing $ENV{DESTDIR}${file}")
    endif()
  else(IS_SYMLINK "$ENV{DESTDIR}${file}" OR EXISTS "$ENV{DESTDIR}${file}")
    message(STATUS "File $ENV{DESTDIR}${file} does not exist.")
  endif()
endforeach()
