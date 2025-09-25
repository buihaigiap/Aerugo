#!/bin/bash

# Production Deployment Script for Aerugo Docker Registry
# Integrated caching, monitoring, and performance optimizations

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
COMPOSE_FILE="docker-compose.production.yml"
ENV_FILE=".env.production"
BACKUP_DIR="./backups"

print_header() {
    echo -e "${BLUE}"
    echo "================================================="
    echo "   🚀 Aerugo Production Deployment Script"
    echo "   High-Performance Docker Registry with Caching"
    echo "================================================="
    echo -e "${NC}"
}

print_step() {
    echo -e "${GREEN}➜${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_step "Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed!"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null && ! command -v docker &> /dev/null; then
        print_error "Docker Compose is not installed!"
        exit 1
    fi
    
    if [[ ! -f "$COMPOSE_FILE" ]]; then
        print_error "Production compose file not found: $COMPOSE_FILE"
        exit 1
    fi
    
    print_step "✅ Prerequisites check passed"
}

# Setup environment variables
setup_environment() {
    print_step "Setting up environment variables..."
    
    if [[ ! -f "$ENV_FILE" ]]; then
        print_warning "Creating production environment file: $ENV_FILE"
        cat > "$ENV_FILE" << EOF
# Database Configuration
POSTGRES_PASSWORD=\$(openssl rand -base64 32)

# Redis Configuration  
REDIS_PASSWORD=\$(openssl rand -base64 32)

# MinIO S3 Storage (for local development)
MINIO_ROOT_USER=aerugo
MINIO_ROOT_PASSWORD=\$(openssl rand -base64 32)

# AWS S3 Configuration (for production)
AWS_ACCESS_KEY_ID=your-aws-access-key
AWS_SECRET_ACCESS_KEY=your-aws-secret-key
AWS_REGION=us-east-1
S3_BUCKET_NAME=aerugo-registry

# Grafana Configuration
GRAFANA_PASSWORD=\$(openssl rand -base64 16)

# Performance Tuning
DATABASE_MAX_CONNECTIONS=100
REDIS_MAX_CONNECTIONS=50
REQUEST_TIMEOUT=300
MAX_CONCURRENT_REQUESTS=1000
EOF
        
        # Generate actual passwords
        sed -i "s/\$(openssl rand -base64 32)/$(openssl rand -base64 32)/g" "$ENV_FILE"
        sed -i "s/\$(openssl rand -base64 16)/$(openssl rand -base64 16)/g" "$ENV_FILE"
        
        print_warning "Please review and update $ENV_FILE with actual AWS credentials!"
        print_warning "Generated passwords are stored in $ENV_FILE"
    fi
    
    # Load environment
    set -a
    source "$ENV_FILE"
    set +a
    
    print_step "✅ Environment variables loaded"
}

# Create necessary directories
setup_directories() {
    print_step "Creating necessary directories..."
    
    mkdir -p "$BACKUP_DIR"
    mkdir -p "./logs"
    mkdir -p "./nginx/ssl"
    mkdir -p "./prometheus"
    mkdir -p "./grafana/provisioning"
    
    print_step "✅ Directories created"
}

# Generate SSL certificates (self-signed for development)
setup_ssl() {
    print_step "Setting up SSL certificates..."
    
    if [[ ! -f "./nginx/ssl/cert.pem" ]]; then
        print_warning "Generating self-signed SSL certificate..."
        openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout ./nginx/ssl/key.pem \
            -out ./nginx/ssl/cert.pem \
            -subj "/C=US/ST=State/L=City/O=Organization/OU=OrgUnit/CN=localhost"
    fi
    
    print_step "✅ SSL certificates ready"
}

# Create monitoring configuration
setup_monitoring() {
    print_step "Setting up monitoring configuration..."
    
    # Prometheus configuration
    cat > "./prometheus/prometheus.yml" << EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  # - "first_rules.yml"

scrape_configs:
  - job_name: 'aerugo-registry'
    static_configs:
      - targets: ['aerugo-registry:9090']
    metrics_path: '/metrics'
    scrape_interval: 5s
    
  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']
      
  - job_name: 'postgres'  
    static_configs:
      - targets: ['postgres:5432']
EOF

    # Grafana datasource provisioning
    mkdir -p "./grafana/provisioning/datasources"
    cat > "./grafana/provisioning/datasources/prometheus.yml" << EOF
apiVersion: 1

datasources:
  - name: Prometheus
    type: prometheus
    url: http://prometheus:9090
    isDefault: true
    access: proxy
EOF

    print_step "✅ Monitoring configuration created"
}

# Create nginx configuration
setup_nginx() {
    print_step "Setting up nginx configuration..."
    
    mkdir -p "./nginx"
    cat > "./nginx/nginx.conf" << EOF
events {
    worker_connections 1024;
}

http {
    upstream aerugo_backend {
        least_conn;
        server aerugo-registry:8000 max_fails=3 fail_timeout=30s;
    }
    
    # Rate limiting
    limit_req_zone \$binary_remote_addr zone=api:10m rate=10r/s;
    
    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain application/json application/octet-stream;
    
    server {
        listen 80;
        server_name localhost;
        return 301 https://\$server_name\$request_uri;
    }
    
    server {
        listen 443 ssl http2;
        server_name localhost;
        
        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        
        # Security headers
        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";
        
        # Docker Registry V2 API
        location /v2/ {
            limit_req zone=api burst=20 nodelay;
            
            proxy_pass http://aerugo_backend;
            proxy_set_header Host \$host;
            proxy_set_header X-Real-IP \$remote_addr;
            proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto \$scheme;
            
            # Timeouts for large uploads
            proxy_connect_timeout 300s;
            proxy_send_timeout 300s;
            proxy_read_timeout 300s;
            
            client_max_body_size 5G;
        }
        
        # Health check
        location /health {
            proxy_pass http://aerugo_backend;
        }
        
        # Metrics (protected)
        location /metrics {
            allow 127.0.0.1;
            deny all;
            proxy_pass http://aerugo_backend:9090;
        }
    }
}
EOF
    
    print_step "✅ Nginx configuration created"
}

# Backup existing data
backup_data() {
    print_step "Creating backup of existing data..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_PATH="$BACKUP_DIR/backup_$TIMESTAMP"
    
    if docker-compose -f "$COMPOSE_FILE" ps | grep -q "Up"; then
        mkdir -p "$BACKUP_PATH"
        
        # Backup PostgreSQL
        if docker-compose -f "$COMPOSE_FILE" exec -T postgres pg_dumpall -U aerugo > "$BACKUP_PATH/postgres_backup.sql" 2>/dev/null; then
            print_step "✅ PostgreSQL backup created"
        fi
        
        # Backup Redis
        if docker-compose -f "$COMPOSE_FILE" exec -T redis redis-cli --no-auth-warning -a "$REDIS_PASSWORD" SAVE > /dev/null 2>&1; then
            docker-compose -f "$COMPOSE_FILE" exec -T redis cat /data/dump.rdb > "$BACKUP_PATH/redis_backup.rdb" 2>/dev/null || true
            print_step "✅ Redis backup created"
        fi
        
        print_step "✅ Backup saved to: $BACKUP_PATH"
    else
        print_step "No running services to backup"
    fi
}

# Deploy application
deploy_application() {
    print_step "Deploying Aerugo  with performance optimizations..."
    
    # Build production image
    print_step "Building production Docker image..."
    docker-compose -f "$COMPOSE_FILE" build --no-cache aerugo-registry
    
    # Start infrastructure services first
    print_step "Starting infrastructure services..."
    docker-compose -f "$COMPOSE_FILE" up -d postgres redis minio
    
    # Wait for services to be healthy
    print_step "Waiting for infrastructure services to be ready..."
    sleep 30
    
    # Start main application
    print_step "Starting Aerugo  application..."
    docker-compose -f "$COMPOSE_FILE" up -d aerugo-registry
    
    # Start supporting services
    print_step "Starting monitoring and load balancer..."
    docker-compose -f "$COMPOSE_FILE" up -d nginx prometheus grafana
    
    print_step "✅ All services deployed successfully!"
}

# Verify deployment
verify_deployment() {
    print_step "Verifying deployment..."
    
    # Wait for services to start
    sleep 10
    
    # Check service health
    local services=("postgres" "redis" "aerugo-registry" "nginx")
    local all_healthy=true
    
    for service in "${services[@]}"; do
        if docker-compose -f "$COMPOSE_FILE" ps "$service" | grep -q "Up"; then
            print_step "✅ $service is running"
        else
            print_error "$service is not running!"
            all_healthy=false
        fi
    done
    
    # Test registry endpoint
    if curl -k -f https://localhost/v2/ >/dev/null 2>&1; then
        print_step "✅ Registry API is accessible"
    else
        print_warning "Registry API test failed - check logs"
        all_healthy=false
    fi
    
    # Test health endpoint
    if curl -k -f https://localhost/health >/dev/null 2>&1; then
        print_step "✅ Health endpoint is accessible"
    else
        print_warning "Health endpoint test failed"
        all_healthy=false
    fi
    
    if $all_healthy; then
        print_step "🎉 Deployment verification passed!"
        print_deployment_info
    else
        print_error "Some services are not healthy. Check logs with:"
        echo "docker-compose -f $COMPOSE_FILE logs"
    fi
}

# Print deployment information
print_deployment_info() {
    echo -e "${GREEN}"
    echo "================================================="
    echo "   🎉 Aerugo  Deployment Complete!"
    echo "================================================="
    echo -e "${NC}"
    echo "🌐 Registry API: https://localhost/v2/"
    echo "🏥 Health Check: https://localhost/health"
    echo "📊 Metrics: http://localhost:9090/metrics"
    echo "📈 Prometheus: http://localhost:9091"
    echo "📊 Grafana: http://localhost:3000 (admin/\$GRAFANA_PASSWORD)"
    echo "💾 MinIO Console: http://localhost:9001"
    echo ""
    echo "🔍 View logs: docker-compose -f $COMPOSE_FILE logs -f"
    echo "⏹️  Stop services: docker-compose -f $COMPOSE_FILE down"
    echo "🗂️  Backup location: $BACKUP_DIR"
    echo ""
    echo -e "${YELLOW}Test Docker Registry:${NC}"
    echo "docker pull alpine:latest"
    echo "docker tag alpine:latest localhost/test/alpine:latest" 
    echo "docker push localhost/test/alpine:latest"
    echo "docker pull localhost/test/alpine:latest"
}

# Main deployment flow
main() {
    print_header
    
    case "${1:-deploy}" in
        "deploy")
            check_prerequisites
            setup_environment
            setup_directories
            setup_ssl
            setup_monitoring
            setup_nginx
            backup_data
            deploy_application
            verify_deployment
            ;;
        "stop")
            print_step "Stopping all services..."
            docker-compose -f "$COMPOSE_FILE" down
            ;;
        "logs")
            docker-compose -f "$COMPOSE_FILE" logs -f "${2:-}"
            ;;
        "status")
            docker-compose -f "$COMPOSE_FILE" ps
            ;;
        "backup")
            backup_data
            ;;
        *)
            echo "Usage: $0 [deploy|stop|logs|status|backup]"
            echo "  deploy: Deploy full production stack (default)"
            echo "  stop:   Stop all services"
            echo "  logs:   View service logs (optionally specify service)"
            echo "  status: Show service status"
            echo "  backup: Create backup of data"
            ;;
    esac
}

# Run main function with all arguments
main "$@"
