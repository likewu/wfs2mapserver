for i in {0..20}
do
  echo $(date "+%Y-%m-%d %H:%M:%S") "running... $i"
  git.exe fetch -v --progress -- "origin"
  #git.exe push --progress  -- "origin" wip-kewu:wip-kewu | complete

  if [[ $? == 0 ]]; then
    break
  fi

  sleep 120s
done