#!/bin/sh

curl -X POST 'http://localhost:9290/actix-example/store' -H 'Content-Type: application/json' -d @./example.json