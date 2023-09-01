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
    echo
}

# Add people
person_data='{"id": 1, "name": "Alice", "food": [{"id": 1, "name": "Pizza"}]}'
make_request "POST" "api/add_person" "$person_data"

person_data='{"id": 2, "name": "Bob", "food": [{"id": 2, "name": "Burger"}]}'
make_request "POST" "api/add_person" "$person_data"

person_data='{"id": 3, "name": "Charlie", "food": [{"id": 3, "name": "Pasta"}, {"id": 4, "name": "Salat"}]}'
make_request "POST" "api/add_person" "$person_data"

person_data='{"id": 4, "name": "David", "food": []}'
make_request "POST" "api/add_person" "$person_data"

# List all people
make_request "GET" "api/list_people"

# List all food
make_request "GET" "api/list_all_food"

# List specific person's food
make_request "GET" "api/list_food/1"
make_request "GET" "api/list_food/2"
make_request "GET" "api/list_food/3"
make_request "GET" "api/list_food/4"

# Add food to a person
food_data='{"id": 4, "name": "Sushi"}'
make_request "POST" "api/add_food/1" "$food_data"

food_data='{"id": 5, "name": "Ice Cream"}'
make_request "POST" "api/add_food/1" "$food_data"

# List specific person's food after additions
make_request "GET" "api/list_food/1"
make_request "GET" "api/list_food/2"

# Remove food from a person
# food_data='{"food_id": 1, "person_id": 1}'

food_data='{
    "food_id": 1,
    "person_id": 2
}'

make_request "POST" "api/remove_food" "$food_data"

# List specific person's food after removal
make_request "GET" "api/list_food/1"

# List specific person's food after removal
make_request "GET" "api/list_food/1"

# Remove a person
person_id_data='2'
make_request "POST" "api/remove_person" "$person_id_data"

# List all people after removal
make_request "GET" "api/list_people"
