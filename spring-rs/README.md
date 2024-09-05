## raw query

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

## postgres query

![image](https://github.com/user-attachments/assets/c8df67f2-f0b3-42c5-a2ea-2ed335d4ea65)

```sh
➜  spring-benchmark git:(master) ✗ wrk -t8 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    36.00ms    7.40ms 137.77ms   80.52%
    Req/Sec     1.03k   234.94     2.38k    87.89%
  247031 requests in 30.09s, 53.24MB read
Requests/sec:   8208.51
Transfer/sec:      1.77MB
➜  spring-benchmark git:(master) ✗ wrk -t7 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  7 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    35.53ms    9.88ms  77.58ms   72.20%
    Req/Sec     1.19k   400.66     7.00k    89.31%
  247734 requests in 30.10s, 53.39MB read
Requests/sec:   8231.19
Transfer/sec:      1.77MB
➜  spring-benchmark git:(master) ✗ wrk -t6 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    40.10ms   13.68ms 134.40ms   75.21%
    Req/Sec     1.25k   529.68    11.19k    89.48%
  223155 requests in 30.09s, 48.10MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   7415.35
Transfer/sec:      1.60MB
➜  spring-benchmark git:(master) ✗ wrk -t5 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  5 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    32.07ms    7.52ms  68.88ms   73.14%
    Req/Sec     1.87k   473.42     4.83k    85.64%
  278112 requests in 30.06s, 59.94MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   9250.40
Transfer/sec:      1.99MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    34.52ms    7.00ms  71.11ms   80.97%
    Req/Sec     2.17k   520.98     6.91k    90.36%
  257449 requests in 30.05s, 55.49MB read
  Socket errors: connect 2, read 0, write 0, timeout 0
Requests/sec:   8568.04
Transfer/sec:      1.85MB
➜  spring-benchmark git:(master) ✗ wrk -t2 -c300 -d30s http://localhost:88/postgres
Running 30s test @ http://localhost:88/postgres
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    35.52ms    9.54ms  88.20ms   78.88%
    Req/Sec     4.20k     1.18k    9.48k    83.78%
  249985 requests in 30.07s, 53.88MB read
  Socket errors: connect 3, read 0, write 0, timeout 0
Requests/sec:   8312.06
Transfer/sec:      1.79MB
```
