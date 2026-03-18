#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BIN="target/debug/teraclio"
TMP_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

pass_count=0
fail_count=0

assert_exact() {
  local actual="$1"
  local expected="$2"
  local context="$3"

  if [[ "$actual" == "$expected" ]]; then
    echo "[pass] $context"
    pass_count=$((pass_count + 1))
  else
    echo "[fail] $context"
    echo "  expected: $expected"
    echo "  got: $actual"
    fail_count=$((fail_count + 1))
  fi
}

assert_contains() {
  local haystack="$1"
  local needle="$2"
  local context="$3"

  if [[ "$haystack" == *"$needle"* ]]; then
    echo "[pass] $context"
    pass_count=$((pass_count + 1))
  else
    echo "[fail] $context"
    echo "  expected to contain: $needle"
    echo "  got: $haystack"
    fail_count=$((fail_count + 1))
  fi
}

run_expected_fail() {
  local expected="$1"
  local context="$2"
  shift 2

  if output=$("$@" 2>&1); then
    echo "[fail] $context"
    echo "  expected command to fail"
    fail_count=$((fail_count + 1))
  else
    if [[ "$output" == *"$expected"* ]]; then
      echo "[pass] $context"
      pass_count=$((pass_count + 1))
    else
      echo "[fail] $context"
      echo "  expected to contain: $expected"
      echo "  got: $output"
      fail_count=$((fail_count + 1))
    fi
  fi
}

cargo build --quiet

echo "Running Teraclio CLI smoke tests"

printf '{"name":"World","items":[1,2,3]}' > "$TMP_DIR/data.json"
printf 'Items: {{ data.items | length }}' > "$TMP_DIR/template.tpl"
out=$("$BIN" --source "$TMP_DIR/data.json" --template "$TMP_DIR/template.tpl")
assert_exact "$out" "Items: 3" "file parse with extension auto-detect"

printf '{{ data.from }}' > "$TMP_DIR/tpl2.tpl"
out=$("$BIN" --source - --format json -t "$TMP_DIR/tpl2.tpl" <<< '{"from":"pipe"}')
assert_exact "$out" "pipe" "stdin mode with explicit format"

if output=$("$BIN" --source - -t "$TMP_DIR/tpl2.tpl" <<< '{"from":"pipe2"}' 2>&1); then
  echo "[fail] stdin requires explicit format"
  echo "  expected command to fail"
  fail_count=$((fail_count + 1))
else
  assert_contains "$output" "When reading from stdin, --format must be specified (json, yaml, or toml)." "stdin requires explicit format"
fi

printf '{"name":"x"}' > "$TMP_DIR/bad.bin"
run_expected_fail "Unsupported input format for file" "unknown extension requires explicit --format" \
  "$BIN" --source "$TMP_DIR/bad.bin" --template "$TMP_DIR/tpl2.tpl"

if output=$("$BIN" --source - --format json --env-vars -t "$TMP_DIR/tpl2.tpl" <<< '[]' 2>&1); then
  echo "[fail] env-vars rejects non-object source"
  echo "  expected command to fail"
  fail_count=$((fail_count + 1))
else
  assert_contains "$output" "Cannot include environment variables: data source must be a JSON object when --env-vars is used." "env-vars rejects non-object source"
fi

printf '{"from":"json"}' > "$TMP_DIR/nofmt"
out=$("$BIN" --source "$TMP_DIR/nofmt" --format json -t "$TMP_DIR/tpl2.tpl")
assert_exact "$out" "json" "explicit format override on no-extension file"

echo "passes: $pass_count, fails: $fail_count"
if (( fail_count > 0 )); then
  exit 1
fi
