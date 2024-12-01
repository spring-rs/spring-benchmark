#!/bin/bash

# Benchmarking function
benchmark_api() {
  local url=$1         # Target URL
  local result_dir=$2  # Directory to store results

  # Benchmark parameters
  THREADS=(2 4 6 8)        # Number of threads
  CONNECTIONS=(100 200 300 400 500) # Number of connections (concurrency)
  DURATION=60              # Duration of each test (in seconds)

  # Check if wrk is installed
  if ! command -v wrk &> /dev/null; then
    echo "wrk is not installed. Please install wrk before running this script."
    return 1
  fi

  echo ">>>>> Benchmark API: ${url}"
  echo ">>>>> Benchmark Result Dir: ${result_dir}"

  mkdir -p "${result_dir}"

  # Start benchmarking
  for thread in "${THREADS[@]}"; do
    for conn in "${CONNECTIONS[@]}"; do
      echo "Benchmarking: threads=${thread}, connections=${conn}, duration=${DURATION}s"
      wrk -t"${thread}" -c"${conn}" -d"${DURATION}s" "${url}" | tee "${result_dir}/result_t${thread}_c${conn}.txt"
      echo "Benchmark completed. Results saved to ${result_dir}/result_t${thread}_c${conn}.txt"
      echo "----------------------------------------"
    done
  done

  echo "All benchmarks for ${url} completed!"
}

benchmark_api "http://localhost:80" "result/spring-boot-raw"
benchmark_api "http://localhost:88" "result/spring-rs-raw"
benchmark_api "http://localhost:80/postgres" "result/spring-boot-pg"
benchmark_api "http://localhost:88/postgres" "result/spring-rs-pg"
