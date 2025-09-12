#!/bin/bash
# High-Performance Docker Registry Test Script
# Tests optimized endpoints with caching and performance monitoring

set -e

# Configuration
REGISTRY_HOST="localhost"
REGISTRY_PORT="8080"
REGISTRY_URL="${REGISTRY_HOST}:${REGISTRY_PORT}"
TEST_IMAGE="aerugo-performance-test"
TEST_TAG="latest"
CONCURRENT_REQUESTS=10
PERFORMANCE_TEST_DURATION=30

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

echo_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
echo_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
echo_error() { echo -e "${RED}[ERROR]${NC} $1"; }
echo_step() { echo -e "${BLUE}[STEP]${NC} $1"; }
echo_perf() { echo -e "${PURPLE}[PERF]${NC} $1"; }

# Performance testing functions
test_cache_performance() {
    echo_step "Testing cache performance..."
    
    # Warm up cache
    echo_info "Warming up cache..."
    for i in {1..5}; do
        curl -s "http://${REGISTRY_URL}/v2/_catalog" > /dev/null
        curl -s "http://${REGISTRY_URL}/v2/${TEST_IMAGE}/tags/list" > /dev/null 2>&1 || true
    done
    
    # Test cached vs uncached performance
    echo_info "Testing catalog endpoint performance..."
    
    # Clear cache first
    curl -s -X POST "http://${REGISTRY_URL}/v2/_aerugo/cache/clear" > /dev/null 2>&1 || true
    
    # Measure uncached request
    UNCACHED_TIME=$(curl -w "%{time_total}" -s -o /dev/null "http://${REGISTRY_URL}/v2/_catalog")
    echo_perf "Uncached catalog request: ${UNCACHED_TIME}s"
    
    # Measure cached request
    CACHED_TIME=$(curl -w "%{time_total}" -s -o /dev/null "http://${REGISTRY_URL}/v2/_catalog")
    echo_perf "Cached catalog request: ${CACHED_TIME}s"
    
    # Calculate improvement
    if [[ $(echo "$UNCACHED_TIME > 0" | bc -l) -eq 1 ]] && [[ $(echo "$CACHED_TIME > 0" | bc -l) -eq 1 ]]; then
        IMPROVEMENT=$(echo "scale=2; ($UNCACHED_TIME - $CACHED_TIME) / $UNCACHED_TIME * 100" | bc -l)
        echo_perf "Cache performance improvement: ${IMPROVEMENT}%"
    fi
}

test_concurrent_performance() {
    echo_step "Testing concurrent request performance..."
    
    # Create temporary file for results
    RESULTS_FILE=$(mktemp)
    
    # Function to make concurrent requests
    make_concurrent_requests() {
        local endpoint=$1
        local count=$2
        local results_file=$3
        
        for i in $(seq 1 $count); do
            {
                local start_time=$(date +%s.%N)
                local response=$(curl -s -w "%{http_code},%{time_total}" -o /dev/null "$endpoint")
                local end_time=$(date +%s.%N)
                local duration=$(echo "$end_time - $start_time" | bc -l)
                echo "$response,$duration" >> "$results_file"
            } &
        done
        
        # Wait for all background jobs to complete
        wait
    }
    
    echo_info "Running $CONCURRENT_REQUESTS concurrent catalog requests..."
    make_concurrent_requests "http://${REGISTRY_URL}/v2/_catalog" $CONCURRENT_REQUESTS "$RESULTS_FILE"
    
    # Analyze results
    if [[ -f "$RESULTS_FILE" ]]; then
        local total_requests=$(wc -l < "$RESULTS_FILE")
        local successful_requests=$(grep "^200" "$RESULTS_FILE" | wc -l)
        local avg_response_time=$(awk -F, '{sum+=$2; count++} END {print sum/count}' "$RESULTS_FILE")
        
        echo_perf "Total requests: $total_requests"
        echo_perf "Successful requests: $successful_requests"
        echo_perf "Success rate: $(echo "scale=2; $successful_requests * 100 / $total_requests" | bc -l)%"
        echo_perf "Average response time: ${avg_response_time}s"
        
        # Clean up
        rm -f "$RESULTS_FILE"
    fi
}

test_memory_usage() {
    echo_step "Testing memory usage and cache statistics..."
    
    # Get initial cache stats
    echo_info "Getting cache statistics..."
    local cache_stats=$(curl -s "http://${REGISTRY_URL}/v2/_aerugo/cache/stats" 2>/dev/null || echo '{"error": "endpoint not available"}')
    echo_perf "Cache stats: $cache_stats"
    
    # Get performance metrics
    echo_info "Getting performance metrics..."
    local metrics=$(curl -s "http://${REGISTRY_URL}/v2/_aerugo/metrics" 2>/dev/null || echo '{"error": "endpoint not available"}')
    echo_perf "Performance metrics: $metrics"
}

test_large_file_performance() {
    echo_step "Testing large file operations..."
    
    # Create a larger test image
    echo_info "Creating large test image..."
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    # Create a Dockerfile with multiple layers
    cat > Dockerfile << 'EOF'
FROM alpine:latest
RUN dd if=/dev/zero of=/largefile1 bs=1M count=10
RUN dd if=/dev/zero of=/largefile2 bs=1M count=10
RUN dd if=/dev/zero of=/largefile3 bs=1M count=10
LABEL test.type="performance"
LABEL test.size="large"
CMD ["echo", "Large performance test image"]
EOF
    
    # Build the image
    local large_image="${TEST_IMAGE}-large"
    if docker build -t "${large_image}:${TEST_TAG}" . > /dev/null 2>&1; then
        echo_info "✓ Large test image built"
        
        # Tag for registry
        docker tag "${large_image}:${TEST_TAG}" "${REGISTRY_URL}/${large_image}:${TEST_TAG}"
        
        # Test push performance
        echo_info "Testing large image push performance..."
        local push_start=$(date +%s.%N)
        if docker push "${REGISTRY_URL}/${large_image}:${TEST_TAG}" > /dev/null 2>&1; then
            local push_end=$(date +%s.%N)
            local push_duration=$(echo "$push_end - $push_start" | bc -l)
            echo_perf "Large image push time: ${push_duration}s"
            
            # Remove local copy and test pull performance
            docker rmi "${REGISTRY_URL}/${large_image}:${TEST_TAG}" > /dev/null 2>&1 || true
            
            echo_info "Testing large image pull performance..."
            local pull_start=$(date +%s.%N)
            if docker pull "${REGISTRY_URL}/${large_image}:${TEST_TAG}" > /dev/null 2>&1; then
                local pull_end=$(date +%s.%N)
                local pull_duration=$(echo "$pull_end - $pull_start" | bc -l)
                echo_perf "Large image pull time: ${pull_duration}s"
            else
                echo_warn "Large image pull failed"
            fi
            
            # Cleanup
            docker rmi "${REGISTRY_URL}/${large_image}:${TEST_TAG}" > /dev/null 2>&1 || true
            docker rmi "${large_image}:${TEST_TAG}" > /dev/null 2>&1 || true
        else
            echo_warn "Large image push failed"
        fi
    else
        echo_warn "Failed to build large test image"
    fi
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$temp_dir"
}

test_api_response_times() {
    echo_step "Testing API endpoint response times..."
    
    local endpoints=(
        "/v2/"
        "/v2/_catalog"
        "/v2/${TEST_IMAGE}/tags/list"
        "/v2/_aerugo/cache/stats"
        "/v2/_aerugo/metrics"
    )
    
    for endpoint in "${endpoints[@]}"; do
        echo_info "Testing endpoint: $endpoint"
        local url="http://${REGISTRY_URL}${endpoint}"
        local response_time=$(curl -w "%{time_total}" -s -o /dev/null "$url" 2>/dev/null || echo "0")
        local http_code=$(curl -w "%{http_code}" -s -o /dev/null "$url" 2>/dev/null || echo "000")
        
        if [[ "$http_code" =~ ^[23] ]]; then
            echo_perf "✓ $endpoint: ${response_time}s (HTTP $http_code)"
        else
            echo_warn "⚠ $endpoint: ${response_time}s (HTTP $http_code)"
        fi
    done
}

run_stress_test() {
    echo_step "Running stress test for ${PERFORMANCE_TEST_DURATION}s..."
    
    local stress_results=$(mktemp)
    local stress_pids=()
    
    # Start multiple stress test workers
    for i in $(seq 1 5); do
        {
            local count=0
            local start_time=$(date +%s)
            local end_time=$((start_time + PERFORMANCE_TEST_DURATION))
            
            while [[ $(date +%s) -lt $end_time ]]; do
                curl -s "http://${REGISTRY_URL}/v2/_catalog" > /dev/null 2>&1 && ((count++))
                curl -s "http://${REGISTRY_URL}/v2/" > /dev/null 2>&1 && ((count++))
                sleep 0.1
            done
            
            echo "worker_${i}:$count" >> "$stress_results"
        } &
        stress_pids+=($!)
    done
    
    # Wait for stress test to complete
    for pid in "${stress_pids[@]}"; do
        wait "$pid"
    done
    
    # Analyze stress test results
    if [[ -f "$stress_results" ]]; then
        local total_requests=0
        while IFS=':' read -r worker count; do
            echo_perf "$worker completed $count requests"
            total_requests=$((total_requests + count))
        done < "$stress_results"
        
        local rps=$(echo "scale=2; $total_requests / $PERFORMANCE_TEST_DURATION" | bc -l)
        echo_perf "Total requests: $total_requests"
        echo_perf "Requests per second: $rps"
        
        rm -f "$stress_results"
    fi
}

benchmark_docker_operations() {
    echo_step "Benchmarking complete Docker workflow..."
    
    # Create benchmark image
    local bench_image="${TEST_IMAGE}-benchmark"
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    cat > Dockerfile << 'EOF'
FROM alpine:latest
LABEL benchmark="true"
LABEL timestamp="$(date)"
RUN echo "Benchmark test at $(date)" > /benchmark.txt
CMD ["cat", "/benchmark.txt"]
EOF
    
    # Measure complete workflow
    local total_start=$(date +%s.%N)
    
    # Build
    local build_start=$(date +%s.%N)
    if docker build -t "${bench_image}:${TEST_TAG}" . > /dev/null 2>&1; then
        local build_end=$(date +%s.%N)
        local build_time=$(echo "$build_end - $build_start" | bc -l)
        echo_perf "Build time: ${build_time}s"
        
        # Tag
        docker tag "${bench_image}:${TEST_TAG}" "${REGISTRY_URL}/${bench_image}:${TEST_TAG}"
        
        # Push
        local push_start=$(date +%s.%N)
        if docker push "${REGISTRY_URL}/${bench_image}:${TEST_TAG}" > /dev/null 2>&1; then
            local push_end=$(date +%s.%N)
            local push_time=$(echo "$push_end - $push_start" | bc -l)
            echo_perf "Push time: ${push_time}s"
            
            # Remove local
            docker rmi "${REGISTRY_URL}/${bench_image}:${TEST_TAG}" > /dev/null 2>&1 || true
            
            # Pull
            local pull_start=$(date +%s.%N)
            if docker pull "${REGISTRY_URL}/${bench_image}:${TEST_TAG}" > /dev/null 2>&1; then
                local pull_end=$(date +%s.%N)
                local pull_time=$(echo "$pull_end - $pull_start" | bc -l)
                echo_perf "Pull time: ${pull_time}s"
                
                # Run
                local run_start=$(date +%s.%N)
                if docker run --rm "${REGISTRY_URL}/${bench_image}:${TEST_TAG}" > /dev/null 2>&1; then
                    local run_end=$(date +%s.%N)
                    local run_time=$(echo "$run_end - $run_start" | bc -l)
                    echo_perf "Run time: ${run_time}s"
                    
                    local total_end=$(date +%s.%N)
                    local total_time=$(echo "$total_end - $total_start" | bc -l)
                    echo_perf "Total workflow time: ${total_time}s"
                else
                    echo_warn "Benchmark run failed"
                fi
            else
                echo_warn "Benchmark pull failed"
            fi
        else
            echo_warn "Benchmark push failed"
        fi
        
        # Cleanup
        docker rmi "${REGISTRY_URL}/${bench_image}:${TEST_TAG}" > /dev/null 2>&1 || true
        docker rmi "${bench_image}:${TEST_TAG}" > /dev/null 2>&1 || true
    else
        echo_warn "Benchmark build failed"
    fi
    
    cd - > /dev/null
    rm -rf "$temp_dir"
}

main() {
    echo_info "🚀 Starting High-Performance Docker Registry Tests"
    echo_info "Registry URL: $REGISTRY_URL"
    echo_info "Performance test duration: ${PERFORMANCE_TEST_DURATION}s"
    echo_info "Concurrent requests: $CONCURRENT_REQUESTS"
    echo_info "========================================\n"
    
    case "${1:-all}" in
        "cache")
            test_cache_performance
            ;;
        "concurrent")
            test_concurrent_performance
            ;;
        "memory")
            test_memory_usage
            ;;
        "large")
            test_large_file_performance
            ;;
        "api")
            test_api_response_times
            ;;
        "stress")
            run_stress_test
            ;;
        "benchmark")
            benchmark_docker_operations
            ;;
        "all"|*)
            test_api_response_times
            test_cache_performance
            test_concurrent_performance
            test_memory_usage
            run_stress_test
            test_large_file_performance
            benchmark_docker_operations
            ;;
    esac
    
    echo_info "\n========================================="
    echo_info "🎉 Performance testing completed!"
    echo_info "========================================="
}

if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "High-Performance Docker Registry Test Script"
    echo "Usage: $0 [test_type]"
    echo ""
    echo "Test types:"
    echo "  cache       - Test caching performance"
    echo "  concurrent  - Test concurrent request handling"
    echo "  memory      - Test memory usage and statistics"
    echo "  large       - Test large file operations"
    echo "  api         - Test API response times"
    echo "  stress      - Run stress test"
    echo "  benchmark   - Benchmark complete workflow"
    echo "  all         - Run all tests (default)"
    exit 0
fi

main "$@"
