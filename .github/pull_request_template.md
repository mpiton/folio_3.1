<!-- PR Title Format: <type>(<scope>): <description> -->
<!-- Examples: feat(api): add user authentication endpoint -->
<!-- feat(web): implement responsive design for the homepage -->
<!-- Types: feat, fix, docs, style, refactor, perf, test, chore -->

## 📋 Description
<!-- Provide a clear description of changes and their purpose -->

## 🔗 Related Issues
<!-- Link issues this PR addresses -->
Fixes #(issue_number)

## 🎯 Type of Change
- [ ] 🐛 Bug fix (non-breaking change fixing an issue)
- [ ] ✨ New feature (non-breaking change adding functionality)
- [ ] 💥 Breaking change (fix or feature causing existing functionality to change)
- [ ] 📝 Documentation update
- [ ] 🎨 Code style update (formatting, renaming)
- [ ] ♻️ Code refactoring (no functional changes)
- [ ] ⚡ Performance improvement
- [ ] ✅ Test addition or update
- [ ] 🔧 Build configuration change
- [ ] 🚀 CI/CD configuration change

## 🧪 Testing
### Test Coverage
- [ ] **API (Rust)** tests added/updated (`cargo test`)
- [ ] **Web (Astro)** E2E tests added/updated (`npm run test:e2e`)
- [ ] Test coverage meets project standards

### Testing Instructions
<!-- Provide step-by-step instructions to test changes -->
1.
2.
3.

## 📸 Screenshots/Recordings
<!-- Include before/after screenshots or screen recordings for UI changes -->
<details>
<summary>Visual Changes</summary>

Before | After
:---:|:---:
[screenshot] | [screenshot]

</details>

## ✅ Pre-merge Checklist
### Code Quality
- [ ] Code follows project style guides and conventions
- [ ] **API (Rust)**: `make check` passes without warnings
- [ ] **API (Rust)**: `make fmt` has been applied
- [ ] **Web (Astro)**: `npm run build` completes successfully
- [ ] No unnecessary `console.log` statements or debug code
- [ ] Proper error handling implemented

### Rust-Specific (API)
- [ ] Code leverages idiomatic Rust (ownership, borrowing, `Result`/`Option`)
- [ ] `unsafe` blocks are justified and well-documented
- [ ] Dependencies in `Cargo.toml` are up-to-date and necessary

### Astro-Specific (Web)
- [ ] Components use strongly-typed props
- [ ] Accessibility features maintained (semantic HTML, ARIA attributes)
- [ ] Assets (images, fonts) are optimized
- [ ] No client-side console errors

### Documentation
- [ ] Public APIs documented
- [ ] Complex logic includes explanatory comments
- [ ] README updated if needed
- [ ] CHANGELOG.md updated

### Security & Performance
- [ ] No hardcoded secrets or API keys
- [ ] Performance impact assessed (API response times, page load speed)
- [ ] Input validation is implemented for all user-provided data

## 🚨 Breaking Changes
<!-- Describe any breaking changes and migration steps -->

## 📝 Additional Notes
<!-- Any other context, decisions, or concerns -->