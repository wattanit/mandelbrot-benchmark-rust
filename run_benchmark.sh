#!/bin/bash

if [ "$2" == "parallel" ]
then
  docker run -it --name benchmark mandelbrot-benchmark-rust $1 /root/output.pgm true
else
  docker run -it --name benchmark mandelbrot-benchmark-rust $1 /root/output.pgm false
fi

docker cp benchmark:/root/output.pgm .
docker rm benchmark


