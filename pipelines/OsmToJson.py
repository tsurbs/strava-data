from collections import deque
import xmltodict
import json

import matplotlib.pyplot as plt
from tqdm import tqdm

OSM_FILE_PATH='inputs/pittsburgh-osm-full.osm'

osm_file = open(OSM_FILE_PATH, 'r').read()
osm_dict = xmltodict.parse(osm_file)

# want a mapping from id to lat/lon and neighbors
graph = {}

node_dict = {}
for node in tqdm(osm_dict['osm']["node"]):
    node_id = node.get('@id')
    lat = node.get('@lat')
    lon = node.get('@lon')
    graph[node_id] = {
        'lat': float(lat),
        'lon': float(lon),
        'neighbors': set(),
        'ways': set()
    }
    node_dict[node_id] = {
        'lat': lat,
        'lon': lon
    }

print("Number of nodes: ", len(node_dict))
print("Example node: ", node_dict[next(iter(node_dict.keys()))])
used_nodes = set()
highway_types = {}
# iterate through all ways in the osm file
for way in tqdm(osm_dict['osm']['way']):
    way_id = way.get('@id')
    tags = way.get('tag', [])
    highway_type = None
    if isinstance(tags, dict):
        highway_type = tags['@v'] if tags['@k'] == 'highway' else None
    for tag in tags:
        if not isinstance(tag, dict):
            continue
        if tag.get('@k') == 'highway':
            highway_type = tag.get('@v')
            break
    if highway_type is None:
        print("No highway type found for way: ", way.get('@id'))
        continue

    if highway_type not in highway_types:
        highway_types[highway_type] = 0
    highway_types[highway_type] += 1

    # get the id of the way
    way_id = way.get('@id')
    if way_id == "1377787764":
        print("found it")

    # get the nodes in the way
    nodes = way.get('nd')

    # get the lat/lon of each node
    lat_lon = []
    prev_node = None
    for node in nodes:
        used_nodes.add(node.get('@ref'))
        graph[node.get('@ref')]['ways'].add(way_id)
        if prev_node is not None:
            graph[node.get('@ref')]['neighbors'].add(prev_node)
            graph[prev_node]['neighbors'].add(node.get('@ref'))
        prev_node = node.get('@ref')
    
used_graph = {}
for node in graph:
    if node in used_nodes:
        used_graph[node] = graph[node]

        used_graph[node]['neighbors'] = list(graph[node]['neighbors'])
        used_graph[node]['ways'] = list(graph[node]['ways'])

print("Final number of nodes in graph: ", len(used_graph))

print("Highway types: ", highway_types)

bar = tqdm(total=len(used_graph))
connected_components = []
untouched_nodes = set(used_graph.keys())
while len(untouched_nodes) != 0:
    node = next(iter(untouched_nodes))
    frontier = deque([node])
    connected_components.append([])
    while len(frontier) != 0:
        node = frontier.popleft()
        if node not in untouched_nodes:
            continue
        bar.update(1)
        untouched_nodes.remove(node)
        for neighbor in graph[node]['neighbors']:
            if neighbor in untouched_nodes:
                frontier.append(neighbor)
        connected_components[-1].append(node)
bar.close()
print([len(component) for component in connected_components])
for key in [aa for a in connected_components[1:] for aa in a]:
    del used_graph[key]
json.dump(used_graph, open('graph.json', 'w'))