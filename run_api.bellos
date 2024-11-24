curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Node_Importance/node_importance' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "node": {
        "coords": [5.0, 5.0, 5.0],
        "segment": 1
    },
    "nodes": [
        {"coords": [4.0, 4.0, 4.0], "segment": 1},
        {"coords": [6.0, 6.0, 6.0], "segment": 1}
    ],
    "important_nodes": {
        "1": [{"coords": [4.0, 4.0, 4.0], "segment": 1}],
        "2": [{"coords": [15.0, 15.0, 15.0], "segment": 2}]
    },
    "adjacent_segments": {
        "1": [2],
        "2": [1]
    },
    "grid_steps": [10.0, 10.0, 10.0],
    "min_segment_coverage": 0.5,
    "auth": {
        "authorization_key": "bellande_web_api_opensource"
    }
}'
echo ""
