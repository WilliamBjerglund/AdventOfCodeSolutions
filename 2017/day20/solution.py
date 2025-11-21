import re
from collections import defaultdict

def parse_input(input_data):
    particles = []
    input_pattern = r'p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>'
    for line in input_data.strip().split("\n"):
        match = re.match(input_pattern, line)
        if match:
            values = list(map(int, match.groups()))
            p = tuple(values[0:3])
            v = tuple(values[3:6])
            a = tuple(values[6:9])
            particles.append(Particle(p, v, a))
    return particles

class Particle:
    def __init__(self, p, v, a):
        self.p = list(p)
        self.v = list(v) 
        self.a = list(a)
    
    def update(self):
        for i in range(3):
            self.v[i] += self.a[i]
            self.p[i] += self.v[i]
    
    def distance(self):
        return abs(self.p[0]) + abs(self.p[1]) + abs(self.p[2])
    
    def __str__(self):
        return f"Particle(p={self.p}, v={self.v}, a={self.a})"

def run_simulation(particles, steps=1000):
    """Run complete simulation and return closest particle index."""
    for _ in range(steps):
        for particle in particles:
            particle.update()

    return min(range(len(particles)), key=lambda i: particles[i].distance())

def simulate_with_collisions(particles, steps=1000):
    for _ in range(steps):

        for particle in particles:
            particle.update()
        
        positions = defaultdict(list)
        for i, p in enumerate(particles):
            positions[tuple(p.p)].append(i)
        particles = [p for i, p in enumerate(particles) if len(positions[tuple(p.p)]) == 1]
    
    return len(particles)

if __name__ == "__main__":
    with open("2017/day20/input.txt", "r") as file:
        input_data = file.read()
    #part 1
    particles1 = parse_input(input_data)
    closest_index = run_simulation(particles1, 1000)
    print("Part 1 - Closest particle:", closest_index)
    #part 2
    particles2 = parse_input(input_data)
    remaining = simulate_with_collisions(particles2, 1000)
    print("Part 2 - Particles Remaining:", remaining)