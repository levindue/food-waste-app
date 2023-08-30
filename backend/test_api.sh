#!/bin/bash

BASE_URL="http://localhost:8080"

function make_request() {
    local method="$1"
    local endpoint="$2"
    local data="$3"

    echo "Sending $method request to $endpoint"
    echo "Data: $data"

    response=$(curl -s -X "$method" "$BASE_URL/$endpoint" -d "$data" -H "Content-Type: application/json")

    echo "Response:"
    echo "$response"
}

# Add a person
make_request "POST" "api/add_person" '{"id": 1, "name": "Alice", "food": []}'

# List all people
make_request "GET" "api/list_people"

# Add food
make_request "POST" "api/add_food" '{"id": 1, "name": "Pizza"}'

# List all food
make_request "GET" "api/list_food"

# List specific person's food
# make_request "GET" "api/list_food/1"

# Remove food
make_request "POST" "api/remove_food" '1'

# Remove person
make_request "POST" "api/remove_person" '1'

# Try accessing an unknown endpoint
make_request "GET" "api/unknown_endpoint"

# Try removing unknown food
make_request "POST" "api/remove_food" '999'

# Try removing unknown person
make_request "POST" "api/remove_person" '999'
