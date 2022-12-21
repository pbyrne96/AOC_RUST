import heapq, re
from time import time
import time

data =[]
with open('d16_input.txt') as f:
    data = f.read().strip().splitlines()

flows = {}
edges = {}

for line in data:
    valves = re.findall('[A-Z][A-Z]', line)
    valve = valves.pop(0)
    flow = re.findall('\d+', line)[0]
    flows[valve] = int(flow)
    edges[valve] = valves

def make_yo_move(state):
    time, pos, opened = state
    s = []
    for next in edges[pos]:
        s.append(tuple([time-1, next, opened]))
    if pos not in opened and flows[pos] != 0:
        s.append(tuple([time-1, pos, opened|frozenset([pos])]))
    return s

def team_elephant(state):
    time, team, opened = state
    human, elephant = list(team)[0]
    s = []
    for next_human in edges[human]:
        for next_elephant in edges[elephant]:
            s.append(tuple([time-1, frozenset([(next_human,next_elephant)]), opened]))
    if human not in opened and flows[human] != 0:
        for next_elephant in edges[elephant]:
            s.append(tuple([time-1, frozenset([(human,next_elephant)]), opened|frozenset([human])]))
    if elephant not in opened and flows[elephant] != 0:
        for next_human in edges[human]:
            s.append(tuple([time-1, frozenset([(next_human,elephant)]), opened|frozenset([elephant])]))
        if human not in opened and flows[human] != 0:
            s.append(tuple([time-1, frozenset([(human,elephant)]), opened|frozenset([elephant])|frozenset([human])]))
    return s

def highest_pressure_search(successors, start):
    frontier = []
    pressure_released = {}
    heapq.heappush(frontier, (0,start))
    pressure_released[start] = 0
    cull = 10000
    culls = list(range(15))
    while frontier:
        current = heapq.heappop(frontier)[1]
        if current[0] == 0:
            return pressure_released[current]
        if current[0] == culls[-1]:
            frontier = heapq.nsmallest(cull,frontier)
            del culls[-1]
        for next in successors(current):
            time, _, opened = next
            pressure = sum([flows[x] for x in opened])
            new_pressure = pressure_released[current] + pressure
            if next not in pressure_released or new_pressure > pressure_released[next]:
                pressure_released[next] = new_pressure
                priority = -(time*1000)-new_pressure
                heapq.heappush(frontier, (priority, next))

    return 'Fail'

make_move_start = tuple([29, 'AA', frozenset()])
elephant_start = tuple([25, frozenset([('AA','AA')]), frozenset()])
start = time.time()
res = highest_pressure_search(make_yo_move, make_move_start)
t1_time = round(time.time()-start)

start_2=time.time()
res_2=highest_pressure_search(team_elephant, elephant_start)
t2_time = round(time.time() - start_2)

print(f'p1 -> {res} took {t1_time} seconds :', f'p2-> {res_2}, took {t2_time} seconds')