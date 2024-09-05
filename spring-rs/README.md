![image](https://github.com/user-attachments/assets/b7f044b3-135e-46ee-a40e-c8da66532c0a)

```sh
➜  spring-benchmark git:(master) wrk -t2 -c100 -d30s http://localhost:88     
Running 30s test @ http://localhost:88
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.47ms    0.98ms  13.18ms   71.72%
    Req/Sec    20.21k     2.85k   27.50k    70.33%
  1206573 requests in 30.06s, 147.29MB read
  Socket errors: connect 0, read 0, write 0, timeout 36
Requests/sec:  40143.45
Transfer/sec:      4.90MB
➜  spring-benchmark git:(master) wrk -t2 -c200 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.95ms    3.42ms 223.85ms   76.13%
    Req/Sec    17.24k     5.66k   34.27k    84.62%
  1026771 requests in 30.03s, 125.34MB read
Requests/sec:  34191.28
Transfer/sec:      4.17MB
➜  spring-benchmark git:(master) wrk -t2 -c300 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.75ms    4.42ms 226.85ms   77.37%
    Req/Sec    16.91k     3.43k   39.45k    93.96%
  1004827 requests in 30.07s, 122.66MB read
  Socket errors: connect 3, read 0, write 0, timeout 0
Requests/sec:  33417.42
Transfer/sec:      4.08MB
➜  spring-benchmark git:(master) wrk -t3 -c300 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  3 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.58ms    3.89ms  48.34ms   70.28%
    Req/Sec    11.57k     2.78k   26.50k    94.53%
  1033514 requests in 30.10s, 126.16MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:  34341.04
Transfer/sec:      4.19MB
➜  spring-benchmark git:(master) wrk -t4 -c300 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.68ms    3.97ms 227.21ms   70.78%
    Req/Sec     8.57k     2.08k   20.76k    93.21%
  1020224 requests in 30.08s, 124.54MB read
  Socket errors: connect 3, read 0, write 0, timeout 0
Requests/sec:  33918.77
Transfer/sec:      4.14MB
➜  spring-benchmark git:(master) wrk -t5 -c300 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.34ms    3.88ms 231.38ms   70.67%
    Req/Sec     7.21k     1.84k   16.84k    93.63%
  1071509 requests in 30.09s, 130.80MB read
  Socket errors: connect 1, read 0, write 0, timeout 0
Requests/sec:  35604.63
Transfer/sec:      4.35MB
➜  spring-benchmark git:(master) wrk -t6 -c300 -d30s http://localhost:88
Running 30s test @ http://localhost:88
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.10ms    4.33ms 230.64ms   72.39%
    Req/Sec     5.50k     1.53k   33.63k    94.41%
  981874 requests in 30.10s, 119.86MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:  32624.84
Transfer/sec:      3.98MB
```

## mysql query

![image](https://github.com/user-attachments/assets/6d2c33c7-0853-44ce-a9fc-ea1a48e21893)

```sh
➜  spring-benchmark git:(master) wrk -t2 -c100 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.32ms    4.93ms  56.60ms   74.55%
    Req/Sec     2.15k   450.29     6.44k    75.33%
  128611 requests in 30.08s, 14.96MB read
Requests/sec:   4275.40
Transfer/sec:    509.37KB
➜  spring-benchmark git:(master) wrk -t2 -c200 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    46.49ms    6.33ms  82.09ms   81.63%
    Req/Sec     2.16k   318.06     5.06k    86.62%
  128560 requests in 30.05s, 14.96MB read
Requests/sec:   4278.04
Transfer/sec:    509.69KB
➜  spring-benchmark git:(master) wrk -t2 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    71.49ms   10.36ms 103.95ms   82.72%
    Req/Sec     2.09k   391.12     6.78k    92.45%
  124111 requests in 30.07s, 14.44MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   4127.82
Transfer/sec:    491.79KB
➜  spring-benchmark git:(master) wrk -t3 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  3 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    72.48ms   10.88ms 177.28ms   84.49%
    Req/Sec     1.38k   250.02     4.49k    90.87%
  123074 requests in 30.10s, 14.32MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   4089.21
Transfer/sec:    487.19KB
➜  spring-benchmark git:(master) wrk -t4 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    72.98ms   11.15ms 107.99ms   84.99%
    Req/Sec     1.02k   243.27     6.24k    96.40%
  121919 requests in 30.10s, 14.19MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   4050.91
Transfer/sec:    482.63KB
➜  spring-benchmark git:(master) wrk -t5 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    70.67ms    9.70ms 117.18ms   86.27%
    Req/Sec   846.27    184.74     5.00k    95.99%
  126117 requests in 30.09s, 14.67MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   4190.80
Transfer/sec:    499.29KB
➜  spring-benchmark git:(master) wrk -t6 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    71.32ms    9.31ms 101.36ms   84.25%
    Req/Sec   698.67    122.01     2.06k    90.37%
  125165 requests in 30.06s, 14.56MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   4164.26
Transfer/sec:    496.13KB
➜  spring-benchmark git:(master) wrk -t7 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  7 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    71.25ms    9.05ms 106.73ms   78.77%
    Req/Sec   591.33     98.77     3.02k    83.28%
  123761 requests in 30.10s, 14.40MB read
Requests/sec:   4111.98
Transfer/sec:    489.90KB
➜  spring-benchmark git:(master) wrk -t8 -c300 -d30s http://localhost:88/mysql
Running 30s test @ http://localhost:88/mysql
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    70.52ms   10.61ms 111.67ms   88.51%
    Req/Sec   526.38    156.55     4.98k    97.62%
  125870 requests in 30.10s, 14.64MB read
Requests/sec:   4182.04
Transfer/sec:    498.25KB
```
