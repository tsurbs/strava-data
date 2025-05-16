import json
from tqdm import tqdm
graph = json.load(open('../../graph.json', 'r'))

max_lon, min_lon, min_lat = -79.85, -80.05, 40.38
max_lat = 2/3 * (max_lon - min_lon) + min_lat


print("Max lon: ", max_lon)
print("Min lon: ", min_lon)
print("Max lat: ", max_lat)
print("Min lat: ", min_lat)

def change_coordinates_lon(coord):
    # convert lat / lon in range of pittsburgh to x / y in 0, 600; 0, 600
    coord = float(coord)
    return (coord - min_lon) * (600 / (max_lon - min_lon))

def change_coordinates_lat(coord):
    # convert lat / lon in range of pittsburgh to x / y in 0, 600; 0, 600
    coord = float(coord)
    return (600 - (coord - min_lat) * (600 / (max_lat - min_lat)))

nodes = []
links = []
for node in tqdm(graph):
    nodes.append({
        'id': node,
        'x': change_coordinates_lon(graph[node]['lon']),
        'y': change_coordinates_lat(graph[node]['lat'])
    })

# inject nodes into {{ nodes }} in template.html
with open('template.html', 'r') as f:
    template = f.read()
template = template.replace('{{ nodes }}', json.dumps(nodes))
with open('graph.html', 'w') as f:
    f.write(template)
