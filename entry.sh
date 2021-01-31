#!/bin/sh
server \
 --host $HOST \
 --port $PORT \
 --loglevel $LOGLEVEL \
 --jwt-key $JWTKEY \
 --db-url $DATABASE_URL