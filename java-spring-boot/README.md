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

## mysql query

![image](https://github.com/user-attachments/assets/de34fb43-fb80-4f01-9d80-e2736353449f)

```sh
➜  spring-benchmark git:(master) wrk -t2 -c100 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    19.41ms   25.56ms 388.47ms   86.00%
    Req/Sec     4.15k     1.38k    9.07k    73.57%
  245471 requests in 30.08s, 27.90MB read
Requests/sec:   8159.73
Transfer/sec:      0.93MB
➜  spring-benchmark git:(master) wrk -t2 -c200 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    32.74ms   36.01ms 402.85ms   85.75%
    Req/Sec     4.18k     1.16k    8.70k    79.43%
  249311 requests in 30.07s, 28.34MB read
Requests/sec:   8292.23
Transfer/sec:      0.94MB
➜  spring-benchmark git:(master) wrk -t2 -c300 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    40.51ms   48.46ms 587.29ms   86.51%
    Req/Sec     4.55k     1.19k   14.29k    75.34%
  270281 requests in 30.04s, 30.72MB read
  Socket errors: connect 38, read 0, write 0, timeout 0
Requests/sec:   8998.85
Transfer/sec:      1.02MB
➜  spring-benchmark git:(master) wrk -t3 -c300 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  3 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    40.48ms   47.99ms 526.21ms   86.49%
    Req/Sec     3.00k     0.92k   11.64k    79.93%
  268245 requests in 30.09s, 30.49MB read
  Socket errors: connect 38, read 0, write 0, timeout 0
Requests/sec:   8914.67
Transfer/sec:      1.01MB
➜  spring-benchmark git:(master) wrk -t4 -c300 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.14ms   46.99ms 512.82ms   86.05%
    Req/Sec     2.12k   733.16     5.51k    72.82%
  251377 requests in 30.01s, 28.57MB read
  Socket errors: connect 42, read 0, write 0, timeout 0
Requests/sec:   8376.72
Transfer/sec:      0.95MB
➜  spring-benchmark git:(master) wrk -t5 -c300 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.00ms   46.95ms 571.79ms   86.15%
    Req/Sec     1.69k   536.42     4.83k    74.50%
  251957 requests in 30.06s, 28.64MB read
  Socket errors: connect 43, read 0, write 0, timeout 0
Requests/sec:   8383.10
Transfer/sec:      0.95MB
➜  spring-benchmark git:(master) wrk -t5 -c400 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  5 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    35.22ms   43.55ms 495.39ms   85.41%
    Req/Sec     1.72k   761.90     5.92k    69.03%
  255709 requests in 30.07s, 29.06MB read
  Socket errors: connect 155, read 0, write 0, timeout 0
Requests/sec:   8504.22
Transfer/sec:      0.97MB
➜  spring-benchmark git:(master) wrk -t6 -c300 -d30s http://localhost:80/mysql
Running 30s test @ http://localhost:80/mysql
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    40.09ms   46.18ms 580.08ms   86.29%
    Req/Sec     1.50k   639.33    15.59k    83.71%
  268486 requests in 30.08s, 30.52MB read
  Socket errors: connect 35, read 0, write 0, timeout 0
Requests/sec:   8926.69
Transfer/sec:      1.01MB
```
