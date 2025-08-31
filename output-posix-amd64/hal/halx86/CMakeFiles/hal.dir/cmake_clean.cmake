file(REMOVE_RECURSE
  "hal.def"
  "hal.dll"
  "hal.pdb"
  "hal_stubs.c"
)

# Per-language clean rules from dependency scanning.
foreach(lang ASM C)
  include(CMakeFiles/hal.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
