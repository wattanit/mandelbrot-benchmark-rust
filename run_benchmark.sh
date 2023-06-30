#!/bin/bash

if [ "$2" == "parallel" ]
then
  time docker run -it --name benchmark mandelbrot-benchmark-rust $1 /root/output.pgm true
else
  time docker run -it --name benchmark mandelbrot-benchmark-rust $1 /root/output.pgm false
fi

docker cp benchmark:/root/output.pgm .
docker rm benchmark


