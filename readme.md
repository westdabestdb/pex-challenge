hello!

you can find the rust code in /src folder
there will be 2 files

main.rs -> implementation with rust shared state
main-redis.rs -> i was in a good mood so i decided to implement this with redis as well

so if you want to test the redis implementation, just rename the file to main.rs

also i have a docker-compose.yml file for you to run the redis server
for some reason, my docker rust runs fine but i am not able to access from outside, so i may need help with that. but you can still run the redis server with `docker-compose up -d`

to run my rust code, simply run `cargo build --release` and then `./target/release/pex-challenge`
this will start the server on 127.0.0.1:8080

then go to frontend folder and run `npm install` and then `npm run dev` for the nextjs frontend

i hardcoded the backend url and port so you dont have to mess with .env file

Also here's my test result:
wrk -t12 -c100 -d10s http://127.0.0.1:8080/next
Running 10s test @ http://127.0.0.1:8080/next
12 threads and 100 connections
Thread Stats Avg Stdev Max +/- Stdev
Latency 1.02ms 140.74us 4.77ms 94.17%
Req/Sec 7.87k 545.98 11.46k 88.61%
949259 requests in 10.10s, 210.63MB read
Requests/sec: 93981.77
Transfer/sec: 20.85MB
