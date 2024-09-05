## raw query

![image](https://github.com/user-attachments/assets/8610f889-3105-4a16-964c-0e899e4eda23)

```sh
➜  spring-benchmark git:(master) wrk -t2 -c100 -d30s http://localhost:80  
Running 30s test @ http://localhost:80
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.27ms   28.53ms 523.85ms   92.55%
    Req/Sec    11.01k     5.44k   20.78k    59.90%
  653486 requests in 30.10s, 78.02MB read
Requests/sec:  21711.34
Transfer/sec:      2.59MB
➜  spring-benchmark git:(master) wrk -t2 -c200 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.21ms    4.79ms  78.19ms   81.28%
    Req/Sec    12.51k     3.12k   23.40k    82.78%
  745232 requests in 30.04s, 88.97MB read
Requests/sec:  24805.60
Transfer/sec:      2.96MB
➜  spring-benchmark git:(master) wrk -t2 -c300 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.91ms    5.99ms  90.96ms   80.42%
    Req/Sec    11.39k     2.87k   28.03k    91.44%
  676983 requests in 30.04s, 80.82MB read
  Socket errors: connect 55, read 0, write 0, timeout 0
Requests/sec:  22539.49
Transfer/sec:      2.69MB
➜  spring-benchmark git:(master) wrk -t3 -c300 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  3 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.27ms    5.57ms  78.78ms   79.34%
    Req/Sec     7.54k     2.06k   19.28k    92.18%
  672330 requests in 30.05s, 80.27MB read
  Socket errors: connect 72, read 0, write 0, timeout 0
Requests/sec:  22376.36
Transfer/sec:      2.67MB
➜  spring-benchmark git:(master) wrk -t4 -c300 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.29ms    5.96ms  84.31ms   78.86%
    Req/Sec     5.47k     1.41k   14.10k    88.53%
  650909 requests in 30.08s, 77.71MB read
  Socket errors: connect 64, read 0, write 0, timeout 0
Requests/sec:  21641.63
Transfer/sec:      2.58MB
➜  spring-benchmark git:(master) wrk -t5 -c300 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.03ms   19.61ms   1.07s    99.17%
    Req/Sec     4.49k     1.25k   16.41k    93.44%
  668696 requests in 30.09s, 79.83MB read
  Socket errors: connect 69, read 0, write 0, timeout 0
Requests/sec:  22224.00
Transfer/sec:      2.65MB
➜  spring-benchmark git:(master) wrk -t6 -c300 -d30s http://localhost:80
Running 30s test @ http://localhost:80
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.83ms    5.55ms  80.57ms   78.20%
    Req/Sec     3.72k     1.01k   20.21k    91.25%
  665094 requests in 30.08s, 79.40MB read
  Socket errors: connect 65, read 0, write 0, timeout 0
Requests/sec:  22111.39
Transfer/sec:      2.64MB
```

## postgres query

![image](https://github.com/user-attachments/assets/c9f8f8ee-faf5-4e67-add5-877200c2d9ca)

```sh
➜  spring-benchmark git:(master) ✗ wrk -t2 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    66.44ms  109.70ms   1.79s    91.30%
    Req/Sec     3.42k     1.69k    8.90k    68.26%
  199923 requests in 30.03s, 42.55MB read
  Socket errors: connect 46, read 0, write 0, timeout 0
Requests/sec:   6657.37
Transfer/sec:      1.42MB
➜  spring-benchmark git:(master) wrk -t3 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  3 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    37.41ms   44.71ms 595.52ms   86.56%
    Req/Sec     3.27k     1.01k   13.91k    74.19%
  291281 requests in 30.09s, 62.00MB read
  Socket errors: connect 38, read 0, write 0, timeout 0
Requests/sec:   9679.59
Transfer/sec:      2.06MB
➜  spring-benchmark git:(master) wrk -t4 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    37.81ms   43.29ms 466.73ms   85.87%
    Req/Sec     2.28k   727.13    10.01k    71.92%
  270397 requests in 30.05s, 57.55MB read
  Socket errors: connect 44, read 0, write 0, timeout 0
Requests/sec:   8999.69
Transfer/sec:      1.92MB
➜  spring-benchmark git:(master) wrk -t5 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    36.75ms   42.11ms 492.87ms   85.97%
    Req/Sec     1.96k   733.57    14.51k    79.68%
  290955 requests in 30.03s, 61.93MB read
  Socket errors: connect 35, read 0, write 0, timeout 0
Requests/sec:   9689.35
Transfer/sec:      2.06MB
➜  spring-benchmark git:(master) wrk -t6 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    37.38ms   42.98ms 580.67ms   86.03%
    Req/Sec     1.55k   339.93     7.37k    84.47%
  277572 requests in 30.10s, 59.08MB read
  Socket errors: connect 41, read 0, write 0, timeout 0
Requests/sec:   9222.97
Transfer/sec:      1.96MB
➜  spring-benchmark git:(master) wrk -t7 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  7 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    38.90ms   43.67ms 537.51ms   85.94%
    Req/Sec     1.26k   399.09     8.77k    75.96%
  264188 requests in 30.10s, 56.23MB read
  Socket errors: connect 35, read 0, write 0, timeout 0
Requests/sec:   8777.32
Transfer/sec:      1.87MB
➜  spring-benchmark git:(master) wrk -t8 -c300 -d30s http://localhost:80/postgres
Running 30s test @ http://localhost:80/postgres
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.09ms   45.58ms 623.66ms   86.37%
    Req/Sec     1.07k   444.44    13.00k    84.43%
  254931 requests in 30.04s, 54.26MB read
  Socket errors: connect 29, read 0, write 0, timeout 0
Requests/sec:   8485.15
Transfer/sec:      1.81MB
```
