# 📦 Bellande Node Importance

## 🧙 Organization Website
- [![Organization Website](https://img.shields.io/badge/Explore%20Our-Website-0099cc?style=for-the-badge)](https://robotics-sensors.github.io)

## 🧙 Organization Github
- [![Organization Github ](https://img.shields.io/badge/Explore%20Our-Github-0099cc?style=for-the-badge)](https://github.com/Robotics-Sensors)

# Author, Creator and Maintainer
- **Ronaldson Bellande**

## Bellande Node Importance Executables & Models
- [![Bellande Node Importance Models & Executables ](https://img.shields.io/badge/Bellande%20Node%20Importance-Models/Executables-0099cc?style=for-the-badge)](https://github.com/Artificial-Intelligence-Computer-Vision/bellande_node_importance_models_executables)

# API HTTP Usability (BELLANDE FORMAT)
```
# Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
# GNU General Public License v3.0 or later

url: https://bellande-robotics-sensors-research-innovation-center.org

endpoint_path:
    bellande_node_importance: /api/Bellande_Node_Importance/node_importance

Bellande_Framework_Access_Key: bellande_web_api_opensource
```

# API HTTP Usability (JSON FORMAT)
```
{
  "license": [
    "Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande",
    "This program is free software: you can redistribute it and/or modify",
    "it under the terms of the GNU General Public License as published by",
    "the Free Software Foundation, either version 3 of the License, or",
    "(at your option) any later version.",
    "",
    "This program is distributed in the hope that it will be useful,",
    "but WITHOUT ANY WARRANTY; without even the implied warranty of",
    "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
    "GNU General Public License for more details.",
    "",
    "You should have received a copy of the GNU General Public License",
    "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    "GNU General Public License v3.0 or later"
  ],
  "url": "https://bellande-robotics-sensors-research-innovation-center.org",
  "endpoint_path": {
    "bellande_node_importance": "/api/Bellande_Node_Importance/node_importance"
  },
  "Bellande_Framework_Access_Key": "bellande_web_api_opensource"
}
```

# API Payload Example
```
{
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
}
```

# 🧙 Website Bellande API Testing 
- [![Website API Testing](https://img.shields.io/badge/Bellande%20API-Testing-0099cc?style=for-the-badge)](https://bellande-robotics-sensors-research-innovation-center.org/api/bellande_node_importance_experiment)
  
# Quick Bellande API Testing
```
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
```

## Website PYPI
- https://pypi.org/project/bellande_node_importance

### Installation
- `$ pip install bellande_node_importance`

### Usage 
```
bellande_node_importance_api \
  --node '{"coords": [5.0, 5.0, 5.0], "segment": 1}' \
  --nodes '[{"coords": [4.0, 4.0, 4.0], "segment": 1}, {"coords": [6.0, 6.0, 6.0], "segment": 1}]' \
  --important-nodes '{"1": [{"coords": [4.0, 4.0, 4.0], "segment": 1}], "2": [{"coords": [15.0, 15.0, 15.0], "segment": 2}]}' \
  --adjacent-segments '{"1": [2], "2": [1]}' \
  --grid-steps '[10.0, 10.0, 10.0]' \
  --min-segment-coverage 0.5
```

### Upgrade (if not upgraded)
- `$ pip install --upgrade bellande_node_importance`

```
Name: bellande_node_importance
Summary: Determines node importance in n-dimensional space based on coverage and connectivity
Home-page: github.com/RonaldsonBellande/bellande_node_importance
Author: Ronaldson Bellande
Author-email: ronaldsonbellande@gmail.com
License: GNU General Public License v3.0
```

## Published Paper
```
Coming Soon
```

## Preprint
- [![Preprint](https://img.shields.io/badge/Preprint-Bellande%20Node%20Importance-0099cc?style=for-the-badge)](https://dapp.orvium.io/deposits/6650ccb8afb407dc8beb0ff2/view)

## License
This Algorithm or Models is distributed under the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/), see [LICENSE](https://github.com/RonaldsonBellande/bellande_node_importance/blob/main/LICENSE) and [NOTICE](https://github.com/RonaldsonBellande/bellande_node_importance/blob/main/LICENSE) for more information.
