### START ENVIRONMENT

#### docker

```
cd docker
./startup
```

wait scylla finish startup

### insert data

```
cd scripts

./example.sh
```

### benchmark

```
ab -H 'X-UOW: fede-local' -c 8 -n 1200000 'http://localhost:9290/actix-example/store?name=asd'
```


### for reproduce thread sanitizer

#### first 

```
rustup override set nightly
```

#### startup app
```
RUST_LOG=debug RUSTFLAGS="sanitizer=thread" cargo run --target x86_64-unknown-linux-gnu
```