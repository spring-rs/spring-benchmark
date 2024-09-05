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
