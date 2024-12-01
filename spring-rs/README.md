## raw query

![image](https://github.com/user-attachments/assets/b7f044b3-135e-46ee-a40e-c8da66532c0a)

```sh
Benchmarking: threads=2, connections=100, duration=60s
Running 1m test @ http://localhost:88
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.97ms    4.47ms  85.72ms   92.09%
    Req/Sec     8.99k     2.50k   14.34k    72.68%
  1073172 requests in 1.00m, 131.00MB read
Requests/sec:  17875.88
Transfer/sec:      2.18MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t2_c100.txt
----------------------------------------
Benchmarking: threads=2, connections=200, duration=60s
Running 1m test @ http://localhost:88
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.01ms    5.81ms 102.24ms   85.35%
    Req/Sec    11.53k     3.53k   22.66k    71.32%
  1376494 requests in 1.00m, 168.03MB read
Requests/sec:  22923.42
Transfer/sec:      2.80MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t2_c200.txt
----------------------------------------
Benchmarking: threads=2, connections=300, duration=60s
Running 1m test @ http://localhost:88
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.56ms    7.94ms 243.97ms   86.79%
    Req/Sec    13.57k     4.90k   24.92k    65.02%
  1617463 requests in 1.00m, 197.44MB read
Requests/sec:  26921.83
Transfer/sec:      3.29MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t2_c300.txt
----------------------------------------
Benchmarking: threads=2, connections=400, duration=60s
Running 1m test @ http://localhost:88
  2 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.52ms    7.98ms 139.23ms   87.27%
    Req/Sec    16.41k     4.80k   26.63k    67.20%
  1957151 requests in 1.00m, 238.91MB read
Requests/sec:  32576.23
Transfer/sec:      3.98MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t2_c400.txt
----------------------------------------
Benchmarking: threads=2, connections=500, duration=60s
Running 1m test @ http://localhost:88
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.23ms   10.37ms 258.12ms   89.30%
    Req/Sec    16.98k     4.81k   28.56k    69.53%
  2020501 requests in 1.00m, 246.64MB read
Requests/sec:  33630.50
Transfer/sec:      4.11MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t2_c500.txt
----------------------------------------
Benchmarking: threads=4, connections=100, duration=60s
Running 1m test @ http://localhost:88
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.98ms    4.15ms 110.04ms   93.36%
    Req/Sec     5.47k     1.69k    8.73k    67.65%
  1306528 requests in 1.00m, 159.49MB read
Requests/sec:  21770.07
Transfer/sec:      2.66MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t4_c100.txt
----------------------------------------
Benchmarking: threads=4, connections=200, duration=60s
Running 1m test @ http://localhost:88
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.85ms    4.77ms 203.30ms   84.22%
    Req/Sec     6.65k     1.88k   12.49k    72.14%
  1587050 requests in 1.00m, 193.73MB read
Requests/sec:  26419.68
Transfer/sec:      3.23MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t4_c200.txt
----------------------------------------
Benchmarking: threads=4, connections=300, duration=60s
Running 1m test @ http://localhost:88
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.79ms    9.54ms 250.61ms   88.08%
    Req/Sec     6.35k     2.58k   13.19k    64.19%
  1513246 requests in 1.00m, 184.72MB read
Requests/sec:  25191.17
Transfer/sec:      3.08MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t4_c300.txt
----------------------------------------
Benchmarking: threads=4, connections=400, duration=60s
Running 1m test @ http://localhost:88
  4 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.33ms    7.07ms 226.62ms   86.24%
    Req/Sec     9.19k     2.54k   14.96k    68.19%
  2192327 requests in 1.00m, 267.62MB read
Requests/sec:  36485.16
Transfer/sec:      4.45MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t4_c400.txt
----------------------------------------
Benchmarking: threads=4, connections=500, duration=60s
Running 1m test @ http://localhost:88
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.29ms   10.26ms 281.57ms   85.88%
    Req/Sec     8.60k     3.11k   15.59k    61.77%
  2049783 requests in 1.00m, 250.22MB read
Requests/sec:  34108.95
Transfer/sec:      4.16MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t4_c500.txt
----------------------------------------
Benchmarking: threads=6, connections=100, duration=60s
Running 1m test @ http://localhost:88
  6 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.12ms    3.55ms 117.78ms   89.68%
    Req/Sec     3.34k     1.06k    8.75k    65.17%
  1195546 requests in 1.00m, 145.94MB read
Requests/sec:  19893.36
Transfer/sec:      2.43MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t6_c100.txt
----------------------------------------
Benchmarking: threads=6, connections=200, duration=60s
Running 1m test @ http://localhost:88
  6 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.08ms    5.99ms 125.27ms   90.04%
    Req/Sec     4.42k     1.45k    7.97k    64.05%
  1580918 requests in 1.00m, 192.98MB read
Requests/sec:  26319.02
Transfer/sec:      3.21MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t6_c200.txt
----------------------------------------
Benchmarking: threads=6, connections=300, duration=60s
Running 1m test @ http://localhost:88
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.67ms    6.81ms 124.52ms   90.65%
    Req/Sec     5.53k     1.51k    8.76k    72.87%
  1979557 requests in 1.00m, 241.65MB read
Requests/sec:  32940.85
Transfer/sec:      4.02MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t6_c300.txt
----------------------------------------
Benchmarking: threads=6, connections=400, duration=60s
Running 1m test @ http://localhost:88
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.09ms    8.34ms 245.44ms   83.07%
    Req/Sec     4.86k     1.48k   13.92k    70.12%
  1738319 requests in 1.00m, 212.20MB read
Requests/sec:  28928.34
Transfer/sec:      3.53MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t6_c400.txt
----------------------------------------
Benchmarking: threads=6, connections=500, duration=60s
Running 1m test @ http://localhost:88
  6 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.17ms    8.44ms 244.57ms   79.34%
    Req/Sec     5.62k     1.71k   14.88k    64.99%
  2009304 requests in 1.00m, 245.28MB read
Requests/sec:  33438.62
Transfer/sec:      4.08MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t6_c500.txt
----------------------------------------
Benchmarking: threads=8, connections=100, duration=60s
Running 1m test @ http://localhost:88
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.15ms    2.44ms  55.28ms   86.09%
    Req/Sec     3.01k   802.94     4.65k    66.62%
  1441050 requests in 1.00m, 175.91MB read
Requests/sec:  23981.62
Transfer/sec:      2.93MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t8_c100.txt
----------------------------------------
Benchmarking: threads=8, connections=200, duration=60s
Running 1m test @ http://localhost:88
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.03ms    5.49ms 116.72ms   88.40%
    Req/Sec     3.32k     1.00k    5.74k    67.21%
  1588617 requests in 1.00m, 193.92MB read
Requests/sec:  26438.86
Transfer/sec:      3.23MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t8_c200.txt
----------------------------------------
Benchmarking: threads=8, connections=300, duration=60s
Running 1m test @ http://localhost:88
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.52ms    9.04ms 176.27ms   90.29%
    Req/Sec     3.19k     0.96k    6.36k    69.79%
  1523315 requests in 1.00m, 185.95MB read
Requests/sec:  25351.24
Transfer/sec:      3.09MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t8_c300.txt
----------------------------------------
Benchmarking: threads=8, connections=400, duration=60s
Running 1m test @ http://localhost:88
  8 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.29ms    9.17ms 241.30ms   87.56%
    Req/Sec     4.02k     1.42k    7.47k    65.47%
  1921029 requests in 1.00m, 234.50MB read
Requests/sec:  31971.34
Transfer/sec:      3.90MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t8_c400.txt
----------------------------------------
Benchmarking: threads=8, connections=500, duration=60s
Running 1m test @ http://localhost:88
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.97ms    8.92ms 129.16ms   82.17%
    Req/Sec     4.33k     1.38k    7.55k    63.21%
  2061481 requests in 1.00m, 251.65MB read
Requests/sec:  34312.41
Transfer/sec:      4.19MB
Benchmark completed. Results saved to result/spring-rs-raw/result_t8_c500.txt
----------------------------------------
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
