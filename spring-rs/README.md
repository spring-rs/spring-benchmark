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
