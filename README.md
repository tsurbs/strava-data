# strava-data

Repo to manage the data for my strava project explorations

```
.
├── README.md # This file
├── graph.json # Graph of the relevant area: the biggest connected component of ways in the osm export of the area
├── inputs
│   ├── pittsburgh-osm-full.osm # osm export of pgh area
│   └── strava_activities.json # Strava api response for all of the activities 
├── pipelines
│   ├── OsmToJson.py # Converts osm to the graph json
│   └── activity-to-routes # Converts each activity to *connected* series of nodes in the graph.json
├── routes
│   └── ... # all of the routes
└── visualizers
    ├── graph-visualizer # Simple visualizer for the nodes on the graph
    │   ├── graph.html
    │   ├── graph_formatter.py
    │   └── template.html
    ├── routes-on-graph # Combines the other two
    │   ├── exec.sh
    │   ├── graph.html
    │   ├── graph_formatter.py
    │   ├── route_formatter.py
    │   ├── routes.html
    │   └── template.html
    └── routes-visualizer # simple visualizer for routes
        ├── route_formatter.py
        ├── routes.html
        └── template.html
```