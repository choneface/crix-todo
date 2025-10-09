#!/usr/bin/env bash
set -euo pipefail

# Optional override: set SKIP_CHANGELOG_CHECK=1 to skip changelog validation locally
: "${SKIP_CHANGELOG_CHECK:=0}"

ensure_change_docs_updated() {
  if [ "${SKIP_CHANGELOG_CHECK}" = "1" ]; then
    echo "Skipping changelog and Cargo.toml modification check (SKIP_CHANGELOG_CHECK=1)."
    return 0
  fi

  current_branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")"

  if [ "${current_branch}" = "main" ]; then
    echo "On main branch; skipping changelog and Cargo.toml modification check."
    return 0
  fi

  git fetch -q origin main --depth=1 || true

  if git rev-parse --verify -q origin/main >/dev/null; then
    base_ref="origin/main"
  else
    base_ref="main"
  fi

  if ! git rev-parse --verify -q "${base_ref}" >/dev/null; then
    echo "Could not find base ref '${base_ref}'. Skipping changelog check."
    return 0
  fi

  echo "Checking that CHANGELOG and Cargo.toml were modified compared to ${base_ref}..."

  changed_files="$(git diff --name-only "${base_ref}"...HEAD || true)"

  if [ -z "${changed_files}" ]; then
    echo "No changes found compared to ${base_ref}. Cannot verify semantic versioning compliance."
    exit 1
  fi

  changelog_found=0
  cargo_found=0

  for f in ${changed_files}; do
    base="$(basename "$f")"

    case "$base" in
      CHANGELOG.md|Changelog.md|CHANGELOG|changelog.md)
        changelog_found=1
        ;;
    esac

    case "$f" in
      */Cargo.toml|Cargo.toml)
        cargo_found=1
        ;;
    esac
  done

  if [ $changelog_found -ne 1 ] || [ $cargo_found -ne 1 ]; then
    echo "Semantic versioning check failed."
    echo "Both a CHANGELOG file and at least one Cargo.toml must be modified in this branch."
    echo
    echo "Changed files compared to ${base_ref}:"
    echo "${changed_files}" | sed 's/^/  /'
    echo
    echo "You can bypass this check locally with SKIP_CHANGELOG_CHECK=1."
    exit 1
  fi

  echo "CHANGELOG and Cargo.toml modifications detected."
}

echo "Checking formatting..."
cargo fmt --check

echo "Verifying changelog and Cargo.toml updates..."
ensure_change_docs_updated

echo "Building..."
cargo build --verbose

echo "Running tests..."
cargo test --verbose

echo "All steps completed successfully."
