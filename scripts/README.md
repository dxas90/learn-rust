# Testing Scripts Documentation

This directory contains comprehensive testing scripts for the learn-go application, adapted from the testing patterns in the [dxas90/learn](https://github.com/dxas90/learn) repository.

## Available Scripts

### 1. smoke-test.sh
**Purpose**: Quick validation that basic deployment is working

**What it does**:
- ✅ Checks cluster health
- ✅ Verifies deployment is available
- ✅ Validates service has cluster IP
- ✅ Tests health endpoint with curl pod

**When to use**: Right after deployment for quick validation

**Usage**:
```bash
./scripts/smoke-test.sh
```

### 2. e2e-test.sh
**Purpose**: Comprehensive end-to-end testing for Kubernetes deployment

**What it does**:
- ✅ Validates container image can be pulled and started
- ✅ Checks Kubernetes resources (deployment, service, pods)
- ✅ Tests all application endpoints:
  - `/healthz` - Health check
  - `/ping` - Simple ping
  - `/` - Root/welcome page
  - `/info` - Application info
  - `/version` - Version endpoint
  - `/echo` - POST endpoint
- ✅ Tests readiness and liveness probes
- ✅ Validates resource constraints
- ✅ Tests pod resilience (delete & recreate)
- ✅ Generates comprehensive test report

**When to use**: For complete validation in CI/CD or before production deployment

**Usage**:
```bash
./scripts/e2e-test.sh
```

### 3. integration-test.sh
**Purpose**: Integration testing for local development and CI

**What it does**:
- ✅ Runs unit tests
- ✅ Tests local Go application (runs app and tests endpoints)
- ✅ Tests Docker container (builds, runs, tests)
- ✅ Tests Kubernetes deployment (port-forward and test)
- ✅ Auto-detects environment (local vs k8s)
- ✅ Generates integration test report

**When to use**:
- Local development to validate changes
- CI pipeline before building Docker images
- Integration testing in any environment

**Usage**:
```bash
# Local environment - tests Go app and Docker
./scripts/integration-test.sh

# Kubernetes environment - tests deployed app
kubectl config use-context <your-context>
./scripts/integration-test.sh
```

## Adaptations from dxas90/learn

### Key Changes Made:

1. **Application Name**: Changed from `learn` to `learn-go`
2. **Pod Labels**: Updated from `app=gitops-k8s` to `app.kubernetes.io/name=learn-go`
3. **Endpoints**: Adapted to match learn-go endpoints:
   - Added `/info` endpoint testing
   - Added `/version` endpoint testing
   - Added `/echo` POST endpoint testing
4. **Health Check Response**: Changed from checking for `alive` to `success` in JSON response
5. **Environment Variables**: Added `GO_ENV=test` and `PORT` configuration
6. **Docker Image**: Changed from scratch-based Node.js to scratch-based Go binary

### Workflow Integration

The scripts are integrated into `.github/workflows/full-workflow.yml`:

```yaml
jobs:
  # 1. Unit tests run first
  test:
    - run: go test ./...

  # 2. Docker image is built
  build:
    needs: [lint, test]
    - docker build with load: true
    - export to tar artifact

  # 3. Kubernetes deployment tests in Kind cluster
  test-deployment:
    needs: [build, helm-test]
    steps:
      # Load image into Kind
      # Deploy with Helm
      # Run smoke test (RUNS HERE - requires deployed app)
      - run: ./scripts/smoke-test.sh
      # Run comprehensive E2E tests (RUNS HERE - requires deployed app)
      - run: ./scripts/e2e-test.sh
      # Health verification
```

**IMPORTANT**:

- ✅ `smoke-test.sh` and `e2e-test.sh` run in CI **AFTER** Helm deployment in Kind cluster
- ✅ `integration-test.sh` is for **LOCAL DEVELOPMENT ONLY** (not in CI pipeline)
- ❌ Do NOT run `smoke-test.sh` or `e2e-test.sh` locally unless you have a Kind cluster with the app deployed

    - docker build...

  # 4. Kubernetes deployment tests
  test-deployment:
    needs: [build, helm-test]
    steps:
      # Load image into Kind
      # Deploy with Helm
      # Run smoke test (NEW)
      - run: ./scripts/smoke-test.sh
      # Run comprehensive E2E tests (NEW)
      - run: ./scripts/e2e-test.sh
      # Health verification
```

## Test Flow

```
┌─────────────────┐
│   Unit Tests    │ (Go test framework)
└────────┬────────┘
         ↓
┌─────────────────┐
│ Integration     │ (Local + Docker)
│ Tests           │ ./scripts/integration-test.sh
└────────┬────────┘
         ↓
┌─────────────────┐
│  Build Docker   │
│     Image       │
└────────┬────────┘
         ↓
┌─────────────────┐
│  Deploy to      │
│   Kind K8s      │
└────────┬────────┘
         ↓
┌─────────────────┐
│  Smoke Test     │ Quick validation
│                 │ ./scripts/smoke-test.sh
└────────┬────────┘
         ↓
┌─────────────────┐
│  E2E Tests      │ Comprehensive validation
│                 │ ./scripts/e2e-test.sh
└────────┬────────┘
         ↓
┌─────────────────┐
│  Deploy to      │
│  Staging/Prod   │
└─────────────────┘
```

## Environment Variables

The scripts respect the following environment variables:

- `APP_NAME`: Application name (default: `learn-go`)
- `NAMESPACE`: Kubernetes namespace (default: `default`)
- `SERVICE_NAME`: Kubernetes service name (default: `learn-go`)
- `DEPLOYMENT_NAME`: Kubernetes deployment name (default: `learn-go`)
- `LOCAL_PORT`: Local port for testing (default: `8080`)
- `GO_ENV`: Go environment (set to `test` for testing)

## Dependencies

### Required Tools:
- `kubectl` - Kubernetes CLI
- `curl` - HTTP client
- `go` - Go programming language
- `docker` - Docker container runtime

### Optional Tools:
- `helm` - For Helm chart testing
- `kind` - For local Kubernetes testing

## Exit Codes

All scripts follow standard exit code conventions:
- `0` - All tests passed
- `1` - Tests failed or error occurred

## Logging

Scripts use color-coded logging:
- 🔵 **BLUE** - Info messages
- 🟢 **GREEN** - Success messages
- 🟡 **YELLOW** - Warning messages
- 🔴 **RED** - Error messages

## Troubleshooting

### Script fails with "permission denied"
```bash
chmod +x scripts/*.sh
```

### Integration test fails locally
Make sure you have Go installed and dependencies downloaded:
```bash
go mod download
```

### Smoke/E2E test fails in Kind
Check if Kind cluster is running:
```bash
kind get clusters
kubectl cluster-info
```

### Pod not found errors
Verify the label selector matches your deployment:
```bash
kubectl get pods -l app.kubernetes.io/name=learn-go
```

## Credits

Testing patterns and scripts adapted from:
- Repository: [dxas90/learn](https://github.com/dxas90/learn)
- Branch: `develop`
- Scripts: `smoke-test.sh`, `e2e-test.sh`, `integration-test.sh`
