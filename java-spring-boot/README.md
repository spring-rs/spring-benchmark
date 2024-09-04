## raw query

![image](https://github.com/user-attachments/assets/49d400e9-9119-4d6c-931d-89eea0fa0d71)

```sh
➜  spring-benchmark git:(master) ✗ wrk -t2 -c500 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.01ms   45.76ms   1.71s    97.88%
    Req/Sec    11.72k     6.42k   35.48k    73.02%
  2784758 requests in 2.00m, 332.47MB read
  Socket errors: connect 270, read 0, write 0, timeout 0
Requests/sec:  23191.27
Transfer/sec:      2.77MB
➜  spring-benchmark git:(master) ✗ wrk -t2 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.56ms    4.51ms  92.25ms   77.11%
    Req/Sec    13.13k     3.26k   27.78k    72.22%
  3131556 requests in 2.00m, 373.88MB read
  Socket errors: connect 774, read 0, write 0, timeout 0
Requests/sec:  26076.90
Transfer/sec:      3.11MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c5000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 5000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.35ms    4.67ms  81.65ms   76.63%
    Req/Sec     5.73k     2.47k   24.82k    72.93%
  2734533 requests in 2.00m, 326.48MB read
  Socket errors: connect 4785, read 0, write 0, timeout 0
Requests/sec:  22771.08
Transfer/sec:      2.72MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c1000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.95ms    4.87ms  91.98ms   76.15%
    Req/Sec     5.76k     2.30k   18.84k    69.13%
  2747846 requests in 2.00m, 328.06MB read
  Socket errors: connect 770, read 0, write 0, timeout 0
Requests/sec:  22889.93
Transfer/sec:      2.73MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c2000 -d120s http://localhost:80
Running 2m test @ http://localhost:80
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.02ms    5.23ms  82.15ms   75.98%
    Req/Sec     5.67k     1.81k   19.05k    78.19%
  2705902 requests in 2.00m, 323.06MB read
  Socket errors: connect 1772, read 0, write 0, timeout 0
Requests/sec:  22531.98
Transfer/sec:      2.69MB
```

## mysql query

![image](https://github.com/user-attachments/assets/35fb9f51-3237-4e39-848d-f6f5e634a799)

```sh
➜  spring-benchmark git:(master) ✗ wrk -t2 -c500 -d120s http://localhost:80/mysql
Running 2m test @ http://localhost:80/mysql
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    41.94ms   67.57ms   1.73s    90.26%
    Req/Sec     3.80k     1.34k   10.91k    74.25%
  900997 requests in 2.00m, 102.41MB read
  Socket errors: connect 250, read 0, write 0, timeout 0
Requests/sec:   7504.09
Transfer/sec:      0.85MB
➜  spring-benchmark git:(master) ✗ wrk -t2 -c1000 -d120s http://localhost:80/mysql
Running 2m test @ http://localhost:80/mysql
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    30.18ms   42.71ms 700.16ms   85.29%
    Req/Sec     4.55k   834.45    10.77k    74.21%
  1085527 requests in 2.00m, 123.39MB read
  Socket errors: connect 733, read 0, write 0, timeout 0
Requests/sec:   9041.37
Transfer/sec:      1.03MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c5000 -d120s http://localhost:80/mysql
Running 2m test @ http://localhost:80/mysql
  4 threads and 5000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    31.50ms   42.64ms 701.43ms   85.69%
    Req/Sec     2.07k     1.27k    8.64k    64.56%
  989202 requests in 2.00m, 112.44MB read
  Socket errors: connect 4741, read 0, write 0, timeout 0
Requests/sec:   8236.64
Transfer/sec:      0.94MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c1000 -d120s http://localhost:80/mysql
Running 2m test @ http://localhost:80/mysql
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    28.14ms   40.32ms 576.57ms   84.57%
    Req/Sec     2.26k   652.99     5.47k    63.22%
  1076670 requests in 2.00m, 122.38MB read
  Socket errors: connect 752, read 0, write 0, timeout 0
Requests/sec:   8965.55
Transfer/sec:      1.02MB
➜  spring-benchmark git:(master) ✗ wrk -t4 -c2000 -d120s http://localhost:80/mysql
Running 2m test @ http://localhost:80/mysql
  4 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    28.74ms   41.51ms 608.03ms   85.36%
    Req/Sec     2.28k   812.99     8.34k    73.96%
  1088016 requests in 2.00m, 123.67MB read
  Socket errors: connect 1740, read 0, write 0, timeout 0
Requests/sec:   9062.26
Transfer/sec:      1.03MB
```
