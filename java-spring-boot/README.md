![无标题](https://github.com/user-attachments/assets/086a6cb3-34a6-42b9-b269-adc80a988376)

## raw query

```sh
>>>>> Benchmark API: http://localhost:80
>>>>> Benchmark Result Dir: result/spring-boot-raw
Benchmarking: threads=2, connections=100, duration=60s
Running 1m test @ http://localhost:80
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.32ms   31.02ms 998.11ms   95.85%
    Req/Sec     9.43k     3.75k   19.74k    72.06%
  1120011 requests in 1.00m, 133.72MB read
Requests/sec:  18641.23
Transfer/sec:      2.23MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t2_c100.txt
----------------------------------------
Benchmarking: threads=2, connections=200, duration=60s
Running 1m test @ http://localhost:80
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.39ms    8.39ms 149.61ms   89.66%
    Req/Sec     9.41k     2.91k   17.69k    70.18%
  1123220 requests in 1.00m, 134.10MB read
Requests/sec:  18698.23
Transfer/sec:      2.23MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t2_c200.txt
----------------------------------------
Benchmarking: threads=2, connections=300, duration=60s
Running 1m test @ http://localhost:80
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.77ms   38.53ms 997.05ms   99.05%
    Req/Sec    12.35k     3.77k   21.28k    64.94%
  1471805 requests in 1.00m, 175.72MB read
Requests/sec:  24499.74
Transfer/sec:      2.92MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t2_c300.txt
----------------------------------------
Benchmarking: threads=2, connections=400, duration=60s
Running 1m test @ http://localhost:80
  2 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.91ms   11.26ms 238.63ms   89.84%
    Req/Sec    13.28k     3.86k   21.40k    69.51%
  1583358 requests in 1.00m, 189.03MB read
Requests/sec:  26380.52
Transfer/sec:      3.15MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t2_c400.txt
----------------------------------------
Benchmarking: threads=2, connections=500, duration=60s
Running 1m test @ http://localhost:80
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    25.70ms   33.49ms   1.12s    96.77%
    Req/Sec    10.48k     3.25k   19.54k    67.87%
  1249665 requests in 1.00m, 149.19MB read
Requests/sec:  20798.37
Transfer/sec:      2.48MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t2_c500.txt
----------------------------------------
Benchmarking: threads=4, connections=100, duration=60s
Running 1m test @ http://localhost:80
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.33ms    4.76ms 110.05ms   89.61%
    Req/Sec     4.29k     1.34k    7.28k    65.79%
  1024300 requests in 1.00m, 122.29MB read
Requests/sec:  17048.10
Transfer/sec:      2.04MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t4_c100.txt
----------------------------------------
Benchmarking: threads=4, connections=200, duration=60s
Running 1m test @ http://localhost:80
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.75ms    8.02ms 121.54ms   90.40%
    Req/Sec     5.07k     1.57k    8.22k    69.44%
  1210064 requests in 1.00m, 144.47MB read
Requests/sec:  20145.17
Transfer/sec:      2.41MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t4_c200.txt
----------------------------------------
Benchmarking: threads=4, connections=300, duration=60s
Running 1m test @ http://localhost:80
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    18.69ms   12.83ms 195.85ms   87.41%
    Req/Sec     4.31k     1.54k    8.49k    64.61%
  1028460 requests in 1.00m, 122.79MB read
Requests/sec:  17112.76
Transfer/sec:      2.04MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t4_c300.txt
----------------------------------------
Benchmarking: threads=4, connections=400, duration=60s
Running 1m test @ http://localhost:80
  4 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    20.75ms   14.63ms 277.55ms   88.69%
    Req/Sec     5.17k     1.94k    9.92k    64.97%
  1232867 requests in 1.00m, 147.19MB read
Requests/sec:  20521.78
Transfer/sec:      2.45MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t4_c400.txt
----------------------------------------
Benchmarking: threads=4, connections=500, duration=60s
Running 1m test @ http://localhost:80
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    21.64ms   12.96ms 217.77ms   84.08%
    Req/Sec     6.05k     1.94k   10.55k    65.80%
  1442574 requests in 1.00m, 172.23MB read
Requests/sec:  24007.99
Transfer/sec:      2.87MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t4_c500.txt
----------------------------------------
Benchmarking: threads=6, connections=100, duration=60s
Running 1m test @ http://localhost:80
  6 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.79ms    7.08ms 137.75ms   91.16%
    Req/Sec     2.35k     0.88k    4.11k    64.15%
  840705 requests in 1.00m, 100.37MB read
Requests/sec:  13989.78
Transfer/sec:      1.67MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t6_c100.txt
----------------------------------------
Benchmarking: threads=6, connections=200, duration=60s
Running 1m test @ http://localhost:80
  6 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.59ms    9.54ms 270.54ms   88.02%
    Req/Sec     2.61k     0.87k    5.07k    66.19%
  934688 requests in 1.00m, 111.59MB read
Requests/sec:  15560.46
Transfer/sec:      1.86MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t6_c200.txt
----------------------------------------
Benchmarking: threads=6, connections=300, duration=60s
Running 1m test @ http://localhost:80
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    16.55ms   11.70ms 247.41ms   87.11%
    Req/Sec     3.27k     1.29k    6.67k    64.34%
  1168198 requests in 1.00m, 139.47MB read
Requests/sec:  19456.33
Transfer/sec:      2.32MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t6_c300.txt
----------------------------------------
Benchmarking: threads=6, connections=400, duration=60s
Running 1m test @ http://localhost:80
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    18.24ms   10.99ms 288.07ms   84.94%
    Req/Sec     3.80k     1.16k    6.85k    66.68%
  1359361 requests in 1.00m, 162.29MB read
Requests/sec:  22621.56
Transfer/sec:      2.70MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t6_c400.txt
----------------------------------------
Benchmarking: threads=6, connections=500, duration=60s
Running 1m test @ http://localhost:80
  6 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.36ms   13.55ms 252.58ms   84.47%
    Req/Sec     3.72k     1.15k    6.82k    67.70%
  1331658 requests in 1.00m, 158.98MB read
Requests/sec:  22165.99
Transfer/sec:      2.65MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t6_c500.txt
----------------------------------------
Benchmarking: threads=8, connections=100, duration=60s
Running 1m test @ http://localhost:80
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.80ms    4.47ms  75.23ms   87.29%
    Req/Sec     1.88k   487.59     3.37k    72.43%
  895820 requests in 1.00m, 106.95MB read
Requests/sec:  14916.27
Transfer/sec:      1.78MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t8_c100.txt
----------------------------------------
Benchmarking: threads=8, connections=200, duration=60s
Running 1m test @ http://localhost:80
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.97ms    7.53ms 130.28ms   88.39%
    Req/Sec     2.44k   745.58     4.36k    67.98%
  1165786 requests in 1.00m, 139.18MB read
Requests/sec:  19403.48
Transfer/sec:      2.32MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t8_c200.txt
----------------------------------------
Benchmarking: threads=8, connections=300, duration=60s
Running 1m test @ http://localhost:80
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.99ms    7.46ms 129.50ms   84.75%
    Req/Sec     2.97k   706.66     4.85k    72.11%
  1417074 requests in 1.00m, 169.18MB read
Requests/sec:  23582.66
Transfer/sec:      2.82MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t8_c300.txt
----------------------------------------
Benchmarking: threads=8, connections=400, duration=60s
Running 1m test @ http://localhost:80
  8 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    19.98ms   11.64ms 235.07ms   83.20%
    Req/Sec     2.62k   820.00     5.81k    68.00%
  1250891 requests in 1.00m, 149.34MB read
Requests/sec:  20818.88
Transfer/sec:      2.49MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t8_c400.txt
----------------------------------------
Benchmarking: threads=8, connections=500, duration=60s
Running 1m test @ http://localhost:80
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.06ms   14.76ms 247.68ms   83.50%
    Req/Sec     2.71k     0.99k    5.65k    66.59%
  1295106 requests in 1.00m, 154.62MB read
Requests/sec:  21550.52
Transfer/sec:      2.57MB
Benchmark completed. Results saved to result/spring-boot-raw/result_t8_c500.txt
----------------------------------------
```

## postgres query

```sh
>>>>> Benchmark API: http://localhost:80/postgres
>>>>> Benchmark Result Dir: result/spring-boot-pg
Benchmarking: threads=2, connections=100, duration=60s
Running 1m test @ http://localhost:80/postgres
  2 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    16.33ms   27.77ms 701.35ms   92.16%
    Req/Sec     4.85k     1.43k    7.88k    69.71%
  575555 requests in 1.00m, 122.51MB read
Requests/sec:   9577.04
Transfer/sec:      2.04MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t2_c100.txt
----------------------------------------
Benchmarking: threads=2, connections=200, duration=60s
Running 1m test @ http://localhost:80/postgres
  2 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    28.33ms   26.72ms 332.95ms   84.99%
    Req/Sec     4.27k     1.33k   11.24k    76.51%
  509273 requests in 1.00m, 108.40MB read
Requests/sec:   8485.57
Transfer/sec:      1.81MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t2_c200.txt
----------------------------------------
Benchmarking: threads=2, connections=300, duration=60s
Running 1m test @ http://localhost:80/postgres
  2 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    39.89ms   42.06ms 635.56ms   86.50%
    Req/Sec     4.87k     1.15k   12.00k    74.46%
  581963 requests in 1.00m, 123.87MB read
Requests/sec:   9686.23
Transfer/sec:      2.06MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t2_c300.txt
----------------------------------------
Benchmarking: threads=2, connections=400, duration=60s
Running 1m test @ http://localhost:80/postgres
  2 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    49.50ms   60.52ms   1.19s    91.55%
    Req/Sec     5.04k     1.24k   10.40k    70.18%
  601294 requests in 1.00m, 127.98MB read
Requests/sec:  10006.96
Transfer/sec:      2.13MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t2_c400.txt
----------------------------------------
Benchmarking: threads=2, connections=500, duration=60s
Running 1m test @ http://localhost:80/postgres
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    61.16ms   65.90ms   1.43s    91.72%
    Req/Sec     4.78k     1.18k    8.12k    69.17%
  570552 requests in 1.00m, 121.44MB read
Requests/sec:   9495.68
Transfer/sec:      2.02MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t2_c500.txt
----------------------------------------
Benchmarking: threads=4, connections=100, duration=60s
Running 1m test @ http://localhost:80/postgres
  4 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.93ms   13.61ms 116.68ms   83.35%
    Req/Sec     2.37k   725.25     6.40k    79.45%
  566355 requests in 1.00m, 120.55MB read
Requests/sec:   9423.68
Transfer/sec:      2.01MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t4_c100.txt
----------------------------------------
Benchmarking: threads=4, connections=200, duration=60s
Running 1m test @ http://localhost:80/postgres
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    27.59ms   26.86ms 365.42ms   84.84%
    Req/Sec     2.24k   532.38     4.67k    69.87%
  535622 requests in 1.00m, 114.01MB read
Requests/sec:   8919.17
Transfer/sec:      1.90MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t4_c200.txt
----------------------------------------
Benchmarking: threads=4, connections=300, duration=60s
Running 1m test @ http://localhost:80/postgres
  4 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.16ms   43.96ms   1.03s    86.89%
    Req/Sec     2.37k   587.91     7.01k    68.95%
  565483 requests in 1.00m, 120.36MB read
Requests/sec:   9414.87
Transfer/sec:      2.00MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t4_c300.txt
----------------------------------------
Benchmarking: threads=4, connections=400, duration=60s
Running 1m test @ http://localhost:80/postgres
  4 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    49.47ms   46.87ms   1.24s    87.33%
    Req/Sec     2.42k   669.89     5.57k    71.93%
  577361 requests in 1.00m, 122.89MB read
Requests/sec:   9615.37
Transfer/sec:      2.05MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t4_c400.txt
----------------------------------------
Benchmarking: threads=4, connections=500, duration=60s
Running 1m test @ http://localhost:80/postgres
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    67.21ms   67.99ms   1.22s    90.85%
    Req/Sec     2.16k   668.71     4.62k    64.07%
  515622 requests in 1.00m, 109.75MB read
Requests/sec:   8581.96
Transfer/sec:      1.83MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t4_c500.txt
----------------------------------------
Benchmarking: threads=6, connections=100, duration=60s
Running 1m test @ http://localhost:80/postgres
  6 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.41ms   13.30ms 100.69ms   83.01%
    Req/Sec     1.58k   362.34     2.93k    73.88%
  567501 requests in 1.00m, 120.79MB read
Requests/sec:   9442.87
Transfer/sec:      2.01MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t6_c100.txt
----------------------------------------
Benchmarking: threads=6, connections=200, duration=60s
Running 1m test @ http://localhost:80/postgres
  6 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    26.01ms   22.69ms 290.70ms   77.78%
    Req/Sec     1.46k   467.35     3.10k    65.35%
  524346 requests in 1.00m, 111.61MB read
Requests/sec:   8729.66
Transfer/sec:      1.86MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t6_c200.txt
----------------------------------------
Benchmarking: threads=6, connections=300, duration=60s
Running 1m test @ http://localhost:80/postgres
  6 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    39.15ms   34.12ms 481.82ms   84.41%
    Req/Sec     1.47k   405.15     2.75k    68.52%
  527164 requests in 1.00m, 112.20MB read
Requests/sec:   8774.59
Transfer/sec:      1.87MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t6_c300.txt
----------------------------------------
Benchmarking: threads=6, connections=400, duration=60s
Running 1m test @ http://localhost:80/postgres
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    59.55ms   47.42ms   1.24s    81.80%
    Req/Sec     1.23k   434.03     2.90k    67.27%
  440227 requests in 1.00m, 93.70MB read
Requests/sec:   7328.27
Transfer/sec:      1.56MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t6_c400.txt
----------------------------------------
Benchmarking: threads=6, connections=500, duration=60s
Running 1m test @ http://localhost:80/postgres
  6 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    68.73ms   63.11ms   1.31s    90.09%
    Req/Sec     1.37k   364.42     4.88k    70.15%
  491628 requests in 1.00m, 104.64MB read
Requests/sec:   8186.97
Transfer/sec:      1.74MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t6_c500.txt
----------------------------------------
Benchmarking: threads=8, connections=100, duration=60s
Running 1m test @ http://localhost:80/postgres
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.09ms   12.26ms 109.65ms   84.00%
    Req/Sec     1.01k   290.70     1.87k    66.97%
  481974 requests in 1.00m, 102.59MB read
Requests/sec:   8023.57
Transfer/sec:      1.71MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t8_c100.txt
----------------------------------------
Benchmarking: threads=8, connections=200, duration=60s
Running 1m test @ http://localhost:80/postgres
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    26.18ms   23.07ms 303.80ms   81.10%
    Req/Sec     1.11k   320.58     2.18k    66.88%
  530691 requests in 1.00m, 112.96MB read
Requests/sec:   8838.35
Transfer/sec:      1.88MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t8_c200.txt
----------------------------------------
Benchmarking: threads=8, connections=300, duration=60s
Running 1m test @ http://localhost:80/postgres
  8 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    38.35ms   37.43ms 569.18ms   86.56%
    Req/Sec     1.18k   291.94     2.02k    69.62%
  561658 requests in 1.00m, 119.55MB read
Requests/sec:   9346.69
Transfer/sec:      1.99MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t8_c300.txt
----------------------------------------
Benchmarking: threads=8, connections=400, duration=60s
Running 1m test @ http://localhost:80/postgres
  8 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    49.92ms   45.40ms 746.19ms   86.98%
    Req/Sec     1.18k   295.69     2.09k    69.71%
  564213 requests in 1.00m, 120.09MB read
Requests/sec:   9387.85
Transfer/sec:      2.00MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t8_c400.txt
----------------------------------------
Benchmarking: threads=8, connections=500, duration=60s
Running 1m test @ http://localhost:80/postgres
  8 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    67.05ms   54.07ms   1.19s    86.84%
    Req/Sec     1.04k   301.30     2.04k    68.69%
  495527 requests in 1.00m, 105.47MB read
Requests/sec:   8248.95
Transfer/sec:      1.76MB
Benchmark completed. Results saved to result/spring-boot-pg/result_t8_c500.txt
----------------------------------------
```
