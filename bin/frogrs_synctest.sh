cargo build --release --bin frogrs_synctest
nohup cargo run --release --bin frogrs_synctest -- --players 2 --check-distance 7 > frogrs_synctest.log 2>&1 &
