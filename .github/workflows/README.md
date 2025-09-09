# GitHub Actions Workflows

Đây là tập hợp các GitHub Actions workflows để tự động hóa CI/CD cho dự án Aerugo container registry.

## 📋 Danh sách Workflows

### 1. CI - Continuous Integration (`ci.yml`)
**Trigger**: Mỗi khi push code hoặc tạo pull request

**Chức năng**:
- ✅ Lint và kiểm tra format code Rust
- ✅ Chạy unit tests với các phiên bản Rust khác nhau
- ✅ Chạy integration tests Python
- ✅ Build và test Docker containers
- ✅ Kiểm tra performance benchmarks
- ✅ Tạo báo cáo code coverage
- ✅ Kiểm tra security với cargo audit

**Jobs chính**:
- `lint-and-security`: Format checking, Clippy, Security audit
- `test-rust`: Unit tests với PostgreSQL, Redis, MinIO
- `test-integration`: Integration tests Python
- `test-docker`: Docker build và container tests
- `test-rust-versions`: Cross-version compatibility (1.70+)
- `benchmark`: Performance testing
- `coverage`: Code coverage với Codecov

### 2. CD - Continuous Deployment (`cd.yml`)
**Trigger**: Push lên main branch hoặc tạo tag release

**Chức năng**:
- 🚀 Build multi-platform Docker images (amd64, arm64)
- 🚀 Push images lên GitHub Container Registry
- 🚀 Tạo GitHub releases với binaries
- 🚀 Deploy tự động lên staging environment
- 🚀 Deploy manual lên production (cần approval)
- 🔒 Security scanning cho Docker images
- 📊 Performance monitoring sau deployment
- 🔄 Rollback mechanism

**Environments**:
- `staging`: Tự động deploy từ main branch
- `production`: Manual approval cho release tags
- `rollback`: Emergency rollback mechanism

### 3. Security - Security Scanning (`security.yml`)
**Trigger**: Push code, pull request, hoặc chạy hàng ngày lúc 2 AM UTC

**Chức năng**:
- 🔍 Rust security audit với cargo-audit
- 🔍 Dependency vulnerability scanning với Snyk
- 🔍 SAST analysis với CodeQL
- 🔍 Security scanning với Semgrep
- 🔍 Secret scanning với GitGuardian và TruffleHog
- 🔍 Docker image security với Trivy và Docker Scout
- 🔍 License compliance checking
- 🔍 Configuration security validation
- 📋 Security policy compliance check

**Outputs**:
- SARIF files upload lên GitHub Security tab
- Security summary reports
- Automated issue creation cho vulnerabilities

### 4. Dependencies - Dependency Updates (`dependencies.yml`)
**Trigger**: Hàng tuần vào thứ 2 lúc 9 AM UTC hoặc manual

**Chức năng**:
- 📦 Tự động update Rust dependencies
- ⚡ Update GitHub Actions versions
- 🐳 Check Docker base image updates
- 🔍 Vulnerability scanning và alerting
- 🔄 Tự động tạo pull requests cho updates
- 🚨 Tạo issues cho security vulnerabilities

**Automation**:
- Tự động tạo PRs cho safe updates
- Security alerts cho vulnerabilities
- Compatibility testing trước khi merge

### 5. Release - Release Automation (`release.yml`)
**Trigger**: Manual workflow dispatch với version input

**Chức năng**:
- 🏷️ Validate version format (semantic versioning)
- 📝 Update version trong Cargo.toml và docs
- 📋 Generate changelog từ git commits
- 🧪 Chạy full test suite
- 🔨 Build release artifacts cho multiple platforms
- 🏷️ Tạo Git tags và GitHub releases
- 🔄 Merge release branch về main
- 📢 Post-release notifications và cleanup

**Release Types**:
- `patch`: Bug fixes (1.0.0 → 1.0.1)
- `minor`: New features (1.0.0 → 1.1.0)
- `major`: Breaking changes (1.0.0 → 2.0.0)
- `prerelease`: Pre-release versions (1.0.0-beta.1)

## 🚀 Cách sử dụng

### Phát triển hàng ngày
1. Tạo branch từ `main`
2. Commit code changes
3. Push branch → CI workflow tự động chạy
4. Tạo Pull Request → CI + Security workflows chạy
5. Review và merge PR → CD workflow deploy lên staging

### Release version mới
1. Vào **Actions** tab trên GitHub
2. Chọn **Release Automation** workflow
3. Click **Run workflow**
4. Nhập version (vd: `1.2.3`) và release type
5. Workflow sẽ tự động:
   - Update version
   - Chạy tests
   - Tạo release artifacts
   - Deploy production (nếu không phải prerelease)

### Monitoring Security
- Security workflows chạy tự động hàng ngày
- Check **Security** tab để xem vulnerabilities
- Review Issues được tạo tự động cho security alerts

## ⚙️ Configuration

### Required Secrets
```
GITHUB_TOKEN          # Tự động có sẵn
SNYK_TOKEN            # Cho Snyk security scanning
GITGUARDIAN_API_KEY   # Cho GitGuardian secret scanning
CODECOV_TOKEN         # Cho code coverage reports
```

### Environment Variables
```
REGISTRY=ghcr.io                    # Container registry
IMAGE_NAME=${{ github.repository }} # Docker image name
CARGO_TERM_COLOR=always            # Rust output coloring
```

### Service Dependencies
Các workflows sử dụng services sau trong testing:
- **PostgreSQL**: Port 5433
- **Redis**: Port 6380  
- **MinIO**: Ports 9001/9002

## 📊 Workflow Status

Có thể xem trạng thái workflows qua:
- **Actions** tab trên GitHub repo
- **Badges** trong README (nếu được thêm)
- **Security** tab cho security findings
- **Pull Requests** cho CI status checks

## 🔧 Customization

### Thêm Environment mới
1. Edit `cd.yml`
2. Thêm job mới với `environment` section
3. Configure deployment steps

### Thêm Security Tool
1. Edit `security.yml` 
2. Thêm job mới với tool của bạn
3. Upload SARIF results nếu có

### Modify Release Process
1. Edit `release.yml`
2. Customize version update logic
3. Add/remove release artifacts

## 🆘 Troubleshooting

### CI Failures
- Check service connectivity (PostgreSQL, Redis, MinIO)
- Verify Rust version compatibility
- Review test failures trong job logs

### CD Issues  
- Check container registry permissions
- Verify environment secrets
- Review deployment target availability

### Security Alerts
- Review SARIF uploads trong Security tab
- Check secret scanning results
- Verify dependency vulnerability reports

### Release Problems
- Validate semantic version format
- Ensure clean working directory
- Check GitHub permissions cho releases

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [Container Registry Guide](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [Security Scanning Tools](https://docs.github.com/en/code-security)
