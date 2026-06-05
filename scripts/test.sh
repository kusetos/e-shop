#!/usr/bin/env bash
set -euo pipefail

BASE="http://localhost:8080"
PASS=0
FAIL=0

check() {
    local desc="$1" expected="$2" actual="$3"
    if [ "$actual" = "$expected" ]; then
        echo "  ✓ $desc"
        PASS=$((PASS + 1))
    else
        echo "  ✗ $desc — expected '$expected', got '$actual'"
        FAIL=$((FAIL + 1))
    fi
}

echo ""
echo "=== catalog ==="

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/products")
check "GET /api/products returns 200" "200" "$STATUS"

COUNT=$(curl -s "$BASE/api/products" | jq 'length')
check "at least one product exists" "true" "$([ "$COUNT" -gt 0 ] && echo true || echo false)"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/products/1")
check "GET /api/products/1 returns 200" "200" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/products/999")
check "GET /api/products/999 returns 404" "404" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/categories")
check "GET /api/categories returns 200" "200" "$STATUS"

echo ""
echo "=== identity ==="

EMAIL="ci_$(date +%s)@example.com"

REGISTER=$(curl -s -X POST "$BASE/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$EMAIL\", \"password\": \"secret123\"}")
check "register returns user_id" "true" "$(echo "$REGISTER" | jq 'has("user_id")')"
check "register returns token"   "true" "$(echo "$REGISTER" | jq 'has("token")')"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/auth/register" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$EMAIL\", \"password\": \"secret123\"}")
check "duplicate register returns 409" "409" "$STATUS"

LOGIN=$(curl -s -X POST "$BASE/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$EMAIL\", \"password\": \"secret123\"}")
TOKEN=$(echo "$LOGIN" | jq -r '.token')
check "login returns token" "true" "$([ -n "$TOKEN" ] && [ "$TOKEN" != "null" ] && echo true || echo false)"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"$EMAIL\", \"password\": \"wrongpass\"}")
check "login wrong password returns 401" "401" "$STATUS"

ME=$(curl -s "$BASE/api/auth/me" -H "Authorization: Bearer $TOKEN")
check "GET /auth/me returns email"    "$EMAIL" "$(echo "$ME" | jq -r '.email')"
check "GET /auth/me has no password"  "false"  "$(echo "$ME" | jq 'has("password")')"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/auth/me")
check "GET /auth/me without token returns 401" "401" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/auth/me" \
    -H "Authorization: Bearer invalid.token.here")
check "GET /auth/me with bad token returns 401" "401" "$STATUS"

echo ""
echo "=== basket ==="

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/basket/1" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"product_id": 2, "name": "Test", "price": 100, "quantity": 1}')
check "POST /basket/:id returns 200" "200" "$STATUS"

BASKET=$(curl -s "$BASE/api/basket/1" -H "Authorization: Bearer $TOKEN")
check "GET /basket/:id has items" "true" "$(echo "$BASKET" | jq '.items | length > 0')"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/basket/1")
check "GET /basket without token returns 401" "401" "$STATUS"

echo ""
echo "=== ordering ==="

USER_ID=$(echo "$LOGIN" | jq -r '.user_id')

ORDER=$(curl -s -X POST "$BASE/api/orders" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"items": [{"product_id": 1, "quantity": 1}]}')
ORDER_ID=$(echo "$ORDER" | jq -r '.id')
check "POST /orders returns id"     "true"    "$([ "$ORDER_ID" != "null" ] && echo true || echo false)"
check "POST /orders status=Pending" "Pending" "$(echo "$ORDER" | jq -r '.status')"
check "POST /orders has items"      "true"    "$(echo "$ORDER" | jq '.items | length > 0')"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/orders" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"items": [{"product_id": 999, "quantity": 1}]}')
check "order with missing product returns 422" "422" "$STATUS"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/orders" \
    -H "Content-Type: application/json" \
    -d '{"items": [{"product_id": 1, "quantity": 1}]}')
check "POST /orders without token returns 401" "401" "$STATUS"

UPDATED=$(curl -s -X PUT "$BASE/api/orders/$ORDER_ID/status" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"status": "Confirmed"}')
check "Pending → Confirmed succeeds" "Confirmed" "$(echo "$UPDATED" | jq -r '.status')"

STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X PUT "$BASE/api/orders/$ORDER_ID/status" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"status": "Pending"}')
check "Confirmed → Pending rejected (422)" "422" "$STATUS"

echo ""
echo "=== kafka / stock ==="

STOCK_BEFORE=$(curl -s "$BASE/api/products/1" | jq '.stock')
curl -s -X POST "$BASE/api/orders" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"items": [{"product_id": 1, "quantity": 1}]}' > /dev/null
sleep 2
STOCK_AFTER=$(curl -s "$BASE/api/products/1" | jq '.stock')
check "stock decremented after order (Kafka)" "true" \
    "$([ "$STOCK_AFTER" -lt "$STOCK_BEFORE" ] && echo true || echo false)"

echo ""
echo "=============================="
echo "  passed: $PASS  failed: $FAIL"
echo "=============================="
[ "$FAIL" -eq 0 ]
