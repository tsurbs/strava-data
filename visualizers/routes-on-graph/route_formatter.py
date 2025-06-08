import json
import os
from tqdm import tqdm

routes_dir = "../../routes"

graph = json.load(open("../../graph.json"))

route_filenmes = os.listdir(routes_dir)

routes = []
for filename in tqdm(route_filenmes):
    ids_route = json.load(open(os.path.join(routes_dir, filename)))
    route = []
    for id in ids_route:
        route.append([id, graph[id]["lat"], graph[id]["lon"]])

    routes.append((filename.split(".")[0], route))
print(len(routes[0][1]))
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
for activity_id, route in tqdm(routes):
    nodes.append([])
    for [id, lat, lon] in route:
        nodes[-1].append({
            'id': activity_id,
            'x': change_coordinates_lon(lon),
            'y': change_coordinates_lat(lat)
        })

# inject nodes into {{ nodes }} in template.html
with open('../dist/graph.html', 'r') as f:
    template = f.read()
template = template.replace('{{ route_nodes }}', json.dumps(nodes))
with open('../dist/routes.html', 'w') as f:
    f.write(template)