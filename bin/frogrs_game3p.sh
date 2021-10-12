nohup cargo run --release --bin frogrs_game -- --port 7000 --players local          --players 127.0.0.1:7001  --players 127.0.0.1:7002 > frogrs.1.log 2>&1 &
nohup cargo run --release --bin frogrs_game -- --port 7001 --players 127.0.0.1:7000 --players local           --players 127.0.0.1:7002 > frogrs.2.log 2>&1 &
nohup cargo run --release --bin frogrs_game -- --port 7002 --players 127.0.0.1:7000 --players 127.0.0.1:7001  --players local          > frogrs.3.log 2>&1 &
