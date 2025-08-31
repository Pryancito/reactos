file(REMOVE_RECURSE
  "hal.def"
  "hal_stubs.c"
  "halmp.dll"
  "halmp.pdb"
)

# Per-language clean rules from dependency scanning.
foreach(lang ASM C)
  include(CMakeFiles/halmp.dir/cmake_clean_${lang}.cmake OPTIONAL)
endforeach()
