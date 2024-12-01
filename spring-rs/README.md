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
>>>>> Benchmark API: http://localhost:88/postgres
>>>>> Benchmark Result Dir: result/spring-rs-pg
Benchmarking: threads=2, connections=100, duration=60s
Running 1m test @ http://localhost:88/postgres
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.67ms    6.27ms 119.75ms   88.82%
    Req/Sec     3.77k     1.14k    6.24k    65.47%
  450409 requests in 1.00m, 97.08MB read
Requests/sec:   7497.66
Transfer/sec:      1.62MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t2_c100.txt
----------------------------------------
Benchmarking: threads=2, connections=200, duration=60s
Running 1m test @ http://localhost:88/postgres
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    22.47ms    7.48ms 123.54ms   85.65%
    Req/Sec     4.51k     1.12k    6.62k    67.00%
  539135 requests in 1.00m, 116.20MB read
Requests/sec:   8974.30
Transfer/sec:      1.93MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t2_c200.txt
----------------------------------------
Benchmarking: threads=2, connections=300, duration=60s
Running 1m test @ http://localhost:88/postgres
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    36.55ms   14.12ms 219.15ms   87.97%
    Req/Sec     4.19k     1.23k    6.60k    63.61%
  499563 requests in 1.00m, 107.67MB read
Requests/sec:   8314.32
Transfer/sec:      1.79MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t2_c300.txt
----------------------------------------
Benchmarking: threads=2, connections=400, duration=60s
Running 1m test @ http://localhost:88/postgres
  2 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    54.93ms   18.08ms 285.43ms   86.44%
    Req/Sec     3.70k     0.96k    5.87k    70.37%
  441112 requests in 1.00m, 95.07MB read
Requests/sec:   7341.55
Transfer/sec:      1.58MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t2_c400.txt
----------------------------------------
Benchmarking: threads=2, connections=500, duration=60s
Running 1m test @ http://localhost:88/postgres
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    61.38ms   22.19ms 327.31ms   87.12%
    Req/Sec     4.13k     1.18k    6.24k    62.94%
  493038 requests in 1.00m, 106.26MB read
Requests/sec:   8214.54
Transfer/sec:      1.77MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t2_c500.txt
----------------------------------------
Benchmarking: threads=4, connections=100, duration=60s
Running 1m test @ http://localhost:88/postgres
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.85ms    4.58ms 141.50ms   89.00%
    Req/Sec     2.15k   519.36     3.30k    68.20%
  514527 requests in 1.00m, 110.90MB read
Requests/sec:   8563.42
Transfer/sec:      1.85MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t4_c100.txt
----------------------------------------
Benchmarking: threads=4, connections=200, duration=60s
Running 1m test @ http://localhost:88/postgres
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    22.13ms    6.66ms 108.05ms   84.45%
    Req/Sec     2.29k   533.36     3.38k    67.46%
  546187 requests in 1.00m, 117.72MB read
Requests/sec:   9095.72
Transfer/sec:      1.96MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t4_c200.txt
----------------------------------------
Benchmarking: threads=4, connections=300, duration=60s
Running 1m test @ http://localhost:88/postgres
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    35.82ms    9.82ms 127.51ms   83.86%
    Req/Sec     2.11k   467.28     3.16k    68.77%
  504861 requests in 1.00m, 108.81MB read
Requests/sec:   8405.94
Transfer/sec:      1.81MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t4_c300.txt
----------------------------------------
Benchmarking: threads=4, connections=400, duration=60s
Running 1m test @ http://localhost:88/postgres
  4 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    45.74ms   13.69ms 153.63ms   81.17%
    Req/Sec     2.21k   570.89     3.45k    65.29%
  527544 requests in 1.00m, 113.70MB read
Requests/sec:   8777.81
Transfer/sec:      1.89MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t4_c400.txt
----------------------------------------
Benchmarking: threads=4, connections=500, duration=60s
Running 1m test @ http://localhost:88/postgres
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    62.43ms   18.22ms 239.52ms   80.05%
    Req/Sec     2.02k   518.01     3.08k    62.23%
  481551 requests in 1.00m, 103.79MB read
Requests/sec:   8014.90
Transfer/sec:      1.73MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t4_c500.txt
----------------------------------------
Benchmarking: threads=6, connections=100, duration=60s
Running 1m test @ http://localhost:88/postgres
  6 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.04ms    4.59ms 102.38ms   85.57%
    Req/Sec     1.35k   351.66     2.19k    68.69%
  485618 requests in 1.00m, 104.67MB read
Requests/sec:   8082.35
Transfer/sec:      1.74MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t6_c100.txt
----------------------------------------
Benchmarking: threads=6, connections=200, duration=60s
Running 1m test @ http://localhost:88/postgres
  6 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    25.23ms    7.98ms 122.57ms   85.34%
    Req/Sec     1.33k   322.07     2.03k    68.22%
  475348 requests in 1.00m, 102.45MB read
Requests/sec:   7910.49
Transfer/sec:      1.70MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t6_c200.txt
----------------------------------------
Benchmarking: threads=6, connections=300, duration=60s
Running 1m test @ http://localhost:88/postgres
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    37.99ms   11.97ms 156.58ms   82.37%
    Req/Sec     1.33k   342.42     2.09k    68.04%
  477272 requests in 1.00m, 102.87MB read
Requests/sec:   7942.47
Transfer/sec:      1.71MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t6_c300.txt
----------------------------------------
Benchmarking: threads=6, connections=400, duration=60s
Running 1m test @ http://localhost:88/postgres
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    48.82ms   14.25ms 156.39ms   82.25%
    Req/Sec     1.37k   336.30     2.23k    66.81%
  489197 requests in 1.00m, 105.44MB read
Requests/sec:   8143.04
Transfer/sec:      1.76MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t6_c400.txt
----------------------------------------
Benchmarking: threads=6, connections=500, duration=60s
Running 1m test @ http://localhost:88/postgres
  6 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    63.83ms   18.83ms 262.21ms   82.78%
    Req/Sec     1.31k   333.34     2.03k    66.60%
  470387 requests in 1.00m, 101.38MB read
Requests/sec:   7828.99
Transfer/sec:      1.69MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t6_c500.txt
----------------------------------------
Benchmarking: threads=8, connections=100, duration=60s
Running 1m test @ http://localhost:88/postgres
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.15ms    5.86ms 129.83ms   88.96%
    Req/Sec     0.94k   263.58     1.52k    63.96%
  447268 requests in 1.00m, 96.40MB read
Requests/sec:   7447.95
Transfer/sec:      1.61MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t8_c100.txt
----------------------------------------
Benchmarking: threads=8, connections=200, duration=60s
Running 1m test @ http://localhost:88/postgres
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    25.54ms    9.73ms 120.13ms   82.54%
    Req/Sec     0.99k   312.31     2.38k    61.03%
  475266 requests in 1.00m, 102.43MB read
Requests/sec:   7908.74
Transfer/sec:      1.70MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t8_c200.txt
----------------------------------------
Benchmarking: threads=8, connections=300, duration=60s
Running 1m test @ http://localhost:88/postgres
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    43.76ms   15.01ms 187.88ms   80.81%
    Req/Sec     0.86k   246.69     1.46k    66.28%
  409222 requests in 1.00m, 88.20MB read
Requests/sec:   6810.64
Transfer/sec:      1.47MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t8_c300.txt
----------------------------------------
Benchmarking: threads=8, connections=400, duration=60s
Running 1m test @ http://localhost:88/postgres
  8 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    51.97ms   16.99ms 211.63ms   81.41%
    Req/Sec     0.97k   270.82     2.43k    67.09%
  465647 requests in 1.00m, 100.36MB read
Requests/sec:   7749.89
Transfer/sec:      1.67MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t8_c400.txt
----------------------------------------
Benchmarking: threads=8, connections=500, duration=60s
Running 1m test @ http://localhost:88/postgres
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    56.00ms   13.58ms 181.54ms   84.08%
    Req/Sec     1.11k   232.57     1.67k    70.21%
  532947 requests in 1.00m, 114.87MB read
Requests/sec:   8872.62
Transfer/sec:      1.91MB
Benchmark completed. Results saved to result/spring-rs-pg/result_t8_c500.txt
----------------------------------------
```
