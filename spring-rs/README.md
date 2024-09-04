![image](https://github.com/user-attachments/assets/54ab354b-52da-4d31-bf57-65568caf2dee)

```sh
➜  spring-benchmark git:(master) wrk -t2 -c500 -d120s http://localhost:88    
Running 2m test @ http://localhost:88
  2 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.05ms    4.22ms 248.18ms   73.35%
    Req/Sec    16.27k     3.62k   41.70k    77.80%
  3880874 requests in 2.00m, 473.74MB read
  Socket errors: connect 201, read 0, write 0, timeout 0
Requests/sec:  32320.94
Transfer/sec:      3.95MB
➜  spring-benchmark git:(master) wrk -t2 -c1000 -d120s http://localhost:88
Running 2m test @ http://localhost:88
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.74ms    4.12ms 247.57ms   72.79%
    Req/Sec    16.81k     6.57k   52.55k    72.70%
  4009215 requests in 2.00m, 489.41MB read
  Socket errors: connect 701, read 0, write 0, timeout 0
Requests/sec:  33383.30
Transfer/sec:      4.08MB
➜  spring-benchmark git:(master) wrk -t4 -c1000 -d120s http://localhost:88
Running 2m test @ http://localhost:88
  4 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.72ms    3.78ms 298.69ms   73.48%
    Req/Sec     9.59k     3.37k   27.92k    77.94%
  4571799 requests in 2.00m, 558.08MB read
  Socket errors: connect 702, read 0, write 0, timeout 0
Requests/sec:  38069.30
Transfer/sec:      4.65MB
➜  spring-benchmark git:(master) wrk -t6 -c2000 -d120s http://localhost:88
Running 2m test @ http://localhost:88
  6 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.55ms    3.83ms 230.17ms   71.52%
    Req/Sec     5.77k     4.10k   26.10k    50.73%
  4132879 requests in 2.00m, 504.50MB read
  Socket errors: connect 1700, read 0, write 0, timeout 0
Requests/sec:  34415.40
Transfer/sec:      4.20MB
```
