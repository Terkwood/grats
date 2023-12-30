#!/bin/bash
sh build.sh --dev && miniserve --port 8888 --interfaces 127.0.0.1 ./dist  --index index.html
