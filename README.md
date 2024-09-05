## build target

| case                 | target                          | size          |
|----------------------|---------------------------------|---------------|
| **java-springboot**  | spring-0.0.1-SNAPSHOT.jar       | 22.25MB       |
| **rust-spring-rs**   | spring-rs(release binary)       | 11.17MB       |

## docker image

| case                                                  | BaseImage                       | ImageSize      |
|-------------------------------------------------------|---------------------------------|----------------|
| [**java-springboot**](./java-spring-boot/Dockerfile)  | openjdk:17-jdk-slim(407.74 MB)  | 429.99MB       |
| [**rust-spring-rs**](./spring-rs/Dockerfile)          | debian:bookworm-slim(74.77 MB)  | 124.55MB       |

## benchmark summary

| CPU                                             | Memory     | Docker                               |
| ----------------------------------------------- | ---------- | -------------------------------------|
| 13th Gen Intel(R) Core(TM) i7-13700H   2.40 GHz | 16.0 GB    | Docker version 25.0.3, build 4debf41 |

| case                                                                          | QPS      | CPU  | Memory  |
|-------------------------------------------------------------------------------|----------|------|---------|
| [**java-springboot-raw-query**](./java-spring-boot/README.md#raw-query)       | 24805.60 | 350% | 234.6MB |
| [**rust-spring-rs-raw-query**](./spring-rs/README.md#raw-query)               | 40143.45 | 150% | 21.2MB  |
| [**java-springboot-jdbc-query**](./java-spring-boot/README.md#postgres-query) | 9679.59  | 416% | 268.1MB |
| [**rust-spring-rs-sqlx-query**](./spring-rs/README.md#postgres-query)         | 9250.40  | 317% | 29.47MB |

> **NOTE**: It is recommended to use the postgresql database as the sqlx backend, because the mysql backend performance of sqlx is very poor.
> The [mysql-benchmark](https://github.com/spring-rs/spring-benchmark/tree/mysql-benchmark) branch has a stress test report.