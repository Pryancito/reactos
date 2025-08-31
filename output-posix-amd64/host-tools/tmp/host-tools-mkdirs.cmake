# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION ${CMAKE_VERSION}) # this file comes with cmake

# If CMAKE_DISABLE_SOURCE_CHANGES is set to true and the source directory is an
# existing directory in our source tree, calling file(MAKE_DIRECTORY) on it
# would cause a fatal error, even though it would be a no-op.
if(NOT EXISTS "/home/moebius/reactos")
  file(MAKE_DIRECTORY "/home/moebius/reactos")
endif()
file(MAKE_DIRECTORY
  "/home/moebius/reactos/output-posix-amd64/host-tools/bin"
  "/home/moebius/reactos/output-posix-amd64/host-tools"
  "/home/moebius/reactos/output-posix-amd64/host-tools/tmp"
  "/home/moebius/reactos/output-posix-amd64/host-tools/src/host-tools-stamp"
  "/home/moebius/reactos/output-posix-amd64/host-tools/src"
  "/home/moebius/reactos/output-posix-amd64/host-tools/src/host-tools-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/moebius/reactos/output-posix-amd64/host-tools/src/host-tools-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/moebius/reactos/output-posix-amd64/host-tools/src/host-tools-stamp${cfgdir}") # cfgdir has leading slash
endif()
