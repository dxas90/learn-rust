# GitHub Actions Workflows Documentation

This directory contains GitHub Actions workflows for automated CI/CD pipelines.

## Overview

The learn-go project uses GitHub Actions for:
- Continuous Integration (CI)
- Security Scanning
- Docker Image Building
- Kubernetes Deployment Testing

## Workflows

### 1. Docker Build and Security Scan (dockerimage.yml)

**File**: `.github/workflows/dockerimage.yml`

**Purpose**: Complete CI/CD pipeline including code quality, testing, security scanning, Docker building, and deployment.

#### Triggers

- **Push**: `main`, `develop` branches
- **Pull Request**: `main`, `develop` branches
- **Tags**: `v*` (e.g., v1.0.0, v1.2.3)

#### Jobs

##### Lint Job
- **Runs on**: `ubuntu-latest`
- **Purpose**: Code quality and style checks
- **Steps**:
  1. Checkout code
  2. Setup Go 1.21
  3. Install linters (golint, staticcheck)
  4. Run `go fmt` (formatting check)
  5. Run `go vet` (static analysis)
  6. Run `golint` (style checking)
  7. Run `staticcheck` (advanced static analysis)
  8. Verify module dependencies

**Duration**: ~1-2 minutes

##### Test Job
- **Runs on**: `ubuntu-latest`
- **Strategy**: Matrix testing across Go 1.20, 1.21, 1.22
- **Purpose**: Run unit and integration tests with coverage
- **Steps**:
  1. Checkout code
  2. Setup Go (matrix version)
  3. Download dependencies
  4. Run tests with coverage
  5. Generate coverage report
  6. Upload coverage to Codecov

**Coverage Target**: >79%
**Duration**: ~2-3 minutes per Go version

##### Security Scan Job
- **Runs on**: `ubuntu-latest`
- **Purpose**: Scan for security vulnerabilities
- **Steps**:
  1. Checkout code
  2. Setup Go 1.21
  3. Download dependencies
  4. Run `govulncheck` (Go vulnerability scanner)
  5. Run `gosec` (Go security scanner)

**Duration**: ~2-3 minutes

##### Build Job
- **Runs on**: `ubuntu-latest`
- **Needs**: `[lint, test]`
- **Permissions**:
  - contents: read
  - packages: write
  - security-events: write
- **Purpose**: Build and push multi-architecture Docker images
- **Steps**:
  1. Checkout code
  2. Setup Docker Buildx (multi-platform support)
  3. Login to GitHub Container Registry (GHCR)
  4. Extract metadata (tags, labels)
  5. Build and push Docker image
     - Platforms: linux/amd64, linux/arm64
     - Tags: branch name, PR number, semver, sha
     - Build args: BUILD_DATE, VCS_REF, VERSION
  6. Run Trivy vulnerability scanner on image
  7. Upload Trivy results to GitHub Security tab

**Image Registry**: `ghcr.io`
**Duration**: ~5-8 minutes

##### Deploy Staging Job
- **Runs on**: `ubuntu-latest`
- **Needs**: `[build]`
- **Condition**: Only on `main` branch
- **Environment**: staging (https://staging.learn-go.example.com)
- **Purpose**: Deploy to staging Kubernetes environment
- **Steps**:
  1. Checkout code
  2. Setup Kind cluster
  3. Deploy application with:
     - 2 replicas
     - Liveness probe: /ping
     - Readiness probe: /healthz
     - LoadBalancer service
  4. Wait for deployment rollout
  5. Verify deployment status

**Duration**: ~5-10 minutes

##### Deploy Production Job
- **Runs on**: `ubuntu-latest`
- **Needs**: `[build]`
- **Condition**: Only on version tags (v*)
- **Environment**: production (https://learn-go.example.com)
- **Purpose**: Deploy to production environment
- **Steps**:
  1. Checkout code
  2. Deploy to production (customizable)

**Note**: Currently a placeholder for actual production deployment

**Duration**: Depends on implementation

---

### 2. Kubernetes Deployment Pipeline (k8s-deployment.yml)

**File**: `.github/workflows/k8s-deployment.yml`

**Purpose**: Validate and test Kubernetes deployments in an isolated environment.

#### Triggers

- **Push**: `main` branch
- **Pull Request**: `main` branch
- **Tags**: `v*`

#### Jobs

##### Validate K8s Job
- **Runs on**: `ubuntu-latest`
- **Purpose**: Validate Kubernetes manifest syntax
- **Steps**:
  1. Checkout code
  2. Install kubeval
  3. Validate all YAML manifests in k8s/ directory

**Duration**: ~1 minute

##### Test Deployment Job
- **Runs on**: `ubuntu-latest`
- **Needs**: `[validate-k8s]`
- **Purpose**: Full end-to-end deployment testing
- **Steps**:
  1. Checkout code
  2. Setup Kind cluster
  3. Build Docker image
  4. Load image into Kind
  5. Wait for cluster to be ready
  6. Display cluster information
  7. Create/apply Kubernetes manifests
  8. Deploy application
  9. Test all endpoints (/, /ping, /healthz)
  10. Show application logs
  11. Cleanup

**Configuration**:
```yaml
Replicas: 2
Container Port: 8080
Environment Variables:
  - GO_ENV: production
  - PORT: 8080
  - HOST: 0.0.0.0
Probes:
  Liveness: GET /ping (every 30s)
  Readiness: GET /healthz (every 10s)
Service Type: ClusterIP
```

**Duration**: ~8-12 minutes

---

## Configuration

### Required Secrets

No secrets are required for basic functionality. The workflows use:
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions
- `CODECOV_TOKEN`: (Optional) For Codecov integration

### Environment Variables

Defined in workflow files:
- `REGISTRY`: `ghcr.io` (GitHub Container Registry)
- `IMAGE_NAME`: `${{ github.repository }}`
- `GO_VERSION`: `1.21`

### Permissions

The workflows require the following permissions:
- `contents: read` - Read repository code
- `packages: write` - Push to GitHub Container Registry
- `security-events: write` - Upload security scan results

---

## Workflow Badges

Add these badges to your README:

```markdown
[![Build and Test](https://github.com/dxas90/learn-go/workflows/Docker%20Build%20and%20Security%20Scan/badge.svg)](https://github.com/dxas90/learn-go/actions)
[![K8s Deployment](https://github.com/dxas90/learn-go/workflows/Kubernetes%20Deployment%20Pipeline/badge.svg)](https://github.com/dxas90/learn-go/actions)
```

---

## Local Testing

### Test Lint Stage

```bash
# Install linters
go install golang.org/x/lint/golint@latest
go install honnef.co/go/tools/cmd/staticcheck@latest

# Run checks
go fmt ./...
go vet ./...
golint ./...
staticcheck ./...
go mod verify
```

### Test Build Stage

```bash
# Build multi-arch image (requires Docker Buildx)
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 -t learn-go:test .

# Or build for current platform
docker build -t learn-go:test .
```

### Test Security Scanning

```bash
# Install security tools
go install golang.org/x/vuln/cmd/govulncheck@latest
go install github.com/securego/gosec/v2/cmd/gosec@latest

# Run scans
govulncheck ./...
gosec -fmt=json -out=gosec-results.json ./...

# Scan Docker image with Trivy
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy:latest image learn-go:test
```

### Test Kubernetes Deployment

```bash
# Install Kind
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.20.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/

# Create cluster
kind create cluster --name test-cluster

# Build and load image
docker build -t learn-go:test .
kind load docker-image learn-go:test --name test-cluster

# Deploy
kubectl apply -f k8s/

# Test
kubectl port-forward svc/learn-go-service 8080:80 &
curl http://localhost:8080/healthz

# Cleanup
kind delete cluster --name test-cluster
```

---

## Troubleshooting

### Workflow Fails on Lint

**Issue**: Code formatting or linting errors

**Solution**:
```bash
# Fix formatting
go fmt ./...

# Check for specific issues
go vet ./...
golint ./...
staticcheck ./...
```

### Workflow Fails on Tests

**Issue**: Test failures or insufficient coverage

**Solution**:
```bash
# Run tests locally
go test ./... -v

# Check coverage
go test ./... -coverprofile=coverage.out
go tool cover -func=coverage.out
```

### Workflow Fails on Security Scan

**Issue**: Vulnerabilities detected

**Solution**:
```bash
# Check for vulnerabilities
govulncheck ./...

# Update dependencies
go get -u ./...
go mod tidy

# Review and fix security issues
gosec ./...
```

### Build Fails to Push

**Issue**: Permission denied when pushing to GHCR

**Solution**:
- Ensure workflow has `packages: write` permission
- Verify GITHUB_TOKEN is active
- Check if package exists and has correct permissions

### Deployment Test Fails

**Issue**: Endpoints not responding in Kind cluster

**Solution**:
```bash
# Check pod status
kubectl get pods -A

# Check logs
kubectl logs -l app=learn-go

# Verify service
kubectl get svc
kubectl describe svc learn-go-service

# Test connectivity
kubectl port-forward svc/learn-go-service 8080:80
curl http://localhost:8080/healthz
```

---

## Best Practices

1. **Branch Protection**: Require workflows to pass before merging
2. **Status Checks**: Enable required status checks for critical jobs
3. **Caching**: Workflows use Go module caching for faster builds
4. **Matrix Testing**: Test across multiple Go versions
5. **Security**: Regular security scans on every PR
6. **Semantic Versioning**: Use tags (v1.0.0) for releases
7. **Environments**: Use GitHub Environments for deployment approvals
8. **Secrets Management**: Never commit secrets; use GitHub Secrets

---

## Monitoring

### View Workflow Runs

- Go to repository → Actions tab
- Click on specific workflow to see runs
- View logs, artifacts, and deployment status

### Artifacts

Workflows may produce:
- Coverage reports
- Security scan results (SARIF format)
- Build logs

### Notifications

Configure notifications:
- Settings → Notifications → Actions
- Set up Slack/Discord webhooks for critical failures

---

## References

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Docker Build Push Action](https://github.com/docker/build-push-action)
- [Kind Documentation](https://kind.sigs.k8s.io/)
- [Trivy Scanner](https://aquasecurity.github.io/trivy/)
- [Go Security Tools](https://go.dev/security/)
