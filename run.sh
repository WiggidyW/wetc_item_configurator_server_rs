#!/bin/bash
./service &
/usr/local/bin/envoy -c envoy.yaml --service-cluster proxy &
wait -n
exit $?
