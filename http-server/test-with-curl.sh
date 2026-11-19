#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

makecurl() {
   echo "$1"
   curl -vv "$1" 2>&1
}

#https://github.com/bats-core/bats-core
main() {
     #make sure binary is built
     lsof -t -i :4221 | xargs kill -9
     cargo build
     #start server
     cargo run >> server.log & 
     tail -f server.log &
     # this should return 404 not found error
     # sleep 5s
     # makecurl http://localhost:4221/abcdefg
     #this should return 200 ok
     sleep 5s
     makecurl http://localhost:4221/ 2 &
    

     wait -n

  

}

main "$@"
