import os
import random

dev_percent = 0.1

partitions = {
    "train": [],
    "dev": []
}

routes_dir = os.path.join(os.path.dirname(__file__), '..', 'routes')
routes = os.listdir(routes_dir)
random.shuffle(routes)

num_dev = int(len(routes) * dev_percent)

for i in range(len(routes)):
    route_id = routes[i]
    if i < num_dev:
        partitions['dev'].append(route_id)
    else:
        partitions['train'].append(route_id)

# write partitions to routes/partitions.yaml file
with open(os.path.join(routes_dir, 'partitions.yaml'), 'w') as f:
    f.write('train:\n')
    for route in partitions['train']:
        f.write(f'  - {route}\n')
    f.write('dev:\n')
    for route in partitions['dev']:
        f.write(f'  - {route}\n')

print(f"Partitioned {len(routes)} routes into {len(partitions['train'])} train and {len(partitions['dev'])} dev.")
