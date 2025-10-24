# SonarQube Setup Guide

## Current Status

The CI/CD workflow has been improved with:

✅ **Official SonarSource Action** - Using `sonarsource/sonarqube-scan-action@master` instead of manual binary downloads
✅ **Enhanced Error Handling** - Added MONGO_URL environment variable for backend tests
✅ **Secrets Validation** - Added checks to ensure SonarQube credentials are configured
✅ **Better Documentation** - Clear guidance on what's needed for SonarQube to work

## Remaining Issue: SonarQube Authorization

The workflow currently fails during SonarQube analysis with:

```
ERROR You're not authorized to analyze this project or the project doesn't exist on SonarQube
and you're not authorized to create it. Please contact an administrator.
```

This indicates one of three problems:

### Problem 1: Projects Don't Exist
The projects `folio-api` and `folio-web` don't exist on your SonarQube server.

**Solution:**
1. Go to your SonarQube dashboard: `https://your-sonarqube-url`
2. Create two new projects:
   - Project Key: `folio-api` (name: "Folio API Backend")
   - Project Key: `folio-web` (name: "Folio Web Frontend")
3. Create an authentication token for your GitHub Actions workflow

### Problem 2: Token Doesn't Have Permissions
The `SONAR_TOKEN` exists but doesn't have permission to analyze or create projects.

**Solution:**
1. Log in to SonarQube as an administrator
2. Navigate to **Administration → Security → Users** (or **Tokens**)
3. Create a new token with:
   - Name: `GitHub Actions - folio_3.1`
   - Scope: `analyze` (minimum required)
   - Optionally: `administer`, `scan` for additional permissions
4. Copy the generated token

### Problem 3: Token or URL is Invalid
The `SONAR_HOST_URL` or `SONAR_TOKEN` is incorrect or the SonarQube server is unreachable.

**Solution:**
1. Verify your SonarQube server is running and accessible
2. Test the connection:
   ```bash
   curl -u YOUR_TOKEN: https://your-sonarqube-url/api/system/status
   ```
   (Should return `"status":"UP"`)
3. Verify the token is correct and hasn't expired

## Setup Steps

### Step 1: Create SonarQube Projects (if they don't exist)

```bash
# Using SonarQube CLI or Web UI
# Create project with key 'folio-api'
sonar-scanner -Dsonar.projectKey=folio-api \
  -Dsonar.projectName="Folio API Backend" \
  -Dsonar.sources=portfolio/api/src

# Create project with key 'folio-web'
sonar-scanner -Dsonar.projectKey=folio-web \
  -Dsonar.projectName="Folio Web Frontend" \
  -Dsonar.sources=portfolio/web/src
```

### Step 2: Create SonarQube Authentication Token

1. In SonarQube Web UI: **My Account → Security → Tokens**
2. Click "Generate" and create a new token
3. Give it a name like `GitHub Actions - folio_3.1`
4. Select scope: `analyze` (minimum), or `administer`, `scan` for more permissions
5. Copy the generated token value

### Step 3: Configure GitHub Repository Secrets

1. Go to your GitHub repository: `https://github.com/mpiton/folio_3.1/settings/secrets/actions`
2. Click **New repository secret**
3. Add two secrets:

   **Secret 1: SONAR_HOST_URL**
   - Name: `SONAR_HOST_URL`
   - Value: `https://your-sonarqube-server.com` (without trailing slash)
   - Example: `https://sonarqube.mycompany.com`

   **Secret 2: SONAR_TOKEN**
   - Name: `SONAR_TOKEN`
   - Value: (paste the token from Step 2)

### Step 4: Verify the Setup

1. Push a change to trigger the workflow:
   ```bash
   git push origin develop
   ```
   Or create a pull request

2. Check the GitHub Actions workflow run:
   - Go to **Actions** tab
   - Click the "SonarQube Analysis" workflow
   - Monitor the logs

3. Expected successful output:
   ```
   ✅ SonarQube secrets are configured
   ...
   INFO Analysis report uploaded
   ```

4. View your analysis results in SonarQube:
   - Backend: `https://your-sonarqube-url/dashboard?id=folio-api`
   - Frontend: `https://your-sonarqube-url/dashboard?id=folio-web`

## Troubleshooting

### Still Getting "You're not authorized" Error?

1. **Verify token hasn't expired**: Generate a new token and update the secret
2. **Check token has analyze scope**:
   - In SonarQube: My Account → Security → Tokens
   - Delete and recreate with `analyze` scope
3. **Verify project keys match exactly**:
   - Should be: `folio-api` and `folio-web` (lowercase, with hyphen)
4. **Check SonarQube server is accessible**:
   ```bash
   curl -I https://your-sonarqube-url
   ```

### HTTP 403 or Connection Refused?

1. Verify `SONAR_HOST_URL` is correct and server is running
2. Check firewall/network rules allow access
3. Test with curl:
   ```bash
   curl -u YOUR_TOKEN: https://your-sonarqube-url/api/system/status
   ```

### Tests Still Failing?

The backend tests should now pass with the `MONGO_URL` environment variable. If they still fail:
1. Check the test logs for specific errors
2. The tests expect a MongoDB server (even for configuration validation)
3. In CI, we're using a test MongoDB URL that's not actually running - this is by design

## Project Configuration Files

### Backend: `portfolio/api/sonar-project.properties`

```properties
sonar.projectKey=folio-api
sonar.projectName=Folio API Backend
sonar.projectVersion=1.0.0
sonar.sources=src
sonar.sourceEncoding=UTF-8

# Exclusions
sonar.exclusions=**/tests/**,**/benches/**,**/target/**,**/*.toml,**/Cargo.lock
sonar.coverage.exclusions=**/tests/**,**/benches/**,**/target/**
```

### Frontend: `portfolio/web/sonar-project.properties`

```properties
sonar.projectKey=folio-web
sonar.projectName=Folio Web Frontend
sonar.projectVersion=1.0.0
sonar.sources=src
sonar.sourceEncoding=UTF-8

# Exclusions
sonar.exclusions=**/*.spec.ts,**/*.test.ts,**/node_modules/**,**/dist/**,**/build/**,**/.astro/**,**/public/**
sonar.coverage.exclusions=**/*.spec.ts,**/*.test.ts,**/node_modules/**,**/.astro/**

# TypeScript specific
sonar.typescript.tsconfigPath=tsconfig.json
```

## Workflow Improvements Made (Commit 5cf3c4f)

### 1. MongoDB Environment Variable
Added `MONGO_URL=mongodb://localhost:27017/test` to the backend test step to prevent configuration errors.

### 2. SonarQube Secrets Validation
Both backend and frontend jobs now validate that secrets are configured before proceeding with analysis.

### 3. Better Error Messages
Clear documentation in workflow logs about what's needed:
- How to set up SonarQube secrets
- Project naming requirements
- Token permission requirements

## Next Steps

1. **Set up SonarQube server** (if not already done)
   - Local: Docker container
   - Cloud: SonarCloud (free option)
   - Enterprise: Your company's SonarQube instance

2. **Create projects and token** using the setup steps above

3. **Configure GitHub repository secrets** with your credentials

4. **Trigger a workflow run** by pushing code or creating a PR

5. **Verify the analysis** by checking the SonarQube dashboard

## Additional Resources

- [SonarQube Documentation](https://docs.sonarqube.org/)
- [SonarSource GitHub Action](https://github.com/sonarsource/sonarqube-scan-action)
- [SonarQube API](https://docs.sonarqube.org/latest/extend/web-api/)
- [SonarQube Security Best Practices](https://docs.sonarqube.org/latest/instance-administration/security/)
