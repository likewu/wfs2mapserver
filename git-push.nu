use std *
log info "running..."


for i in 0..20 {
  print -n "running..." $i "\n"
  do -i -c { git.exe fetch -v --progress -- "origin" }
  #git.exe push --progress  -- "origin" wip-kewu:wip-kewu | complete

  if $env.LAST_EXIT_CODE == 0 { break; }

  sleep 120sec
}