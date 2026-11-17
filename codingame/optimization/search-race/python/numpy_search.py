import sys
import math
import numpy as np
import scipy
from numpy import dot, cross
from numpy.linalg import norm
from itertools import product

SEARCH_COS = tuple(product(
    range(-18, 19, 3),
    [0, 10, 20, 40, 80, 120, 160, 200],
))
SEARCH_COS2 = tuple(product(
    [math.radians(x) for x in range(-18, 19, 3)],
    [0, 5, 10, 20, 200],
))

A = np.array

EPSILON = 0.00001
LAPS = 3
WIDTH = 16000
HEIGHT = 9000
CHECKPOINTRADIUS = 600
MAX_ROTATION_PER_TURN = math.pi / 10
CAR_MAX_THRUST = 200
CAR_FRICTION = 0.15
CAR_FRICTION_REST = 1 - CAR_FRICTION

n = int(input())
checkpoints = [A([int(j) for j in input().split()]) for i in range(n)]
checkpoints += [checkpoints[-1]] * 20


def next_move(x, y, vx, vy, angle, a, delta_angle):
    new_x = x + vx
    new_y = y + vy

    new_angle = math.radians(angle + delta_angle)

    ax = math.cos(new_angle) * a
    ay = math.sin(new_angle) * a

    new_vx = math.trunc((vx + ax) * (1 - CAR_FRICTION))
    new_vy = math.trunc((vy + ay) * (1 - CAR_FRICTION))

    return new_x, new_y, new_vx, new_vy, round(math.degrees(new_angle))


def fast_next_move(p, v, angle, a_norm, delta_angle):
    new_p = p + v
    new_angle = angle + delta_angle
    a = A([math.cos(new_angle), math.sin(new_angle)]) * a_norm
    new_v = (v + a) * CAR_FRICTION_REST
    return new_p, new_v, new_angle


def proportion(a, b, t):
    return a + (b - a) * t


def distance_point_to_line(p, a, b):
    v = b - a
    va = p - a
    vb = p - b

    dot_ap = dot(va, v)
    dot_bp = dot(vb, v)

    if dot_ap <= 0:
        return dot(va, va)**.5
    if dot_bp >= 0:
        return dot(vb, vb)**.5

    return norm(cross(v, va)) / norm(v)


def find_best_dist(x, y, vx, vy, angle, checkpoint_index):
    t0 = checkpoints[checkpoint_index]
    t1 = checkpoints[checkpoint_index + 1]
    t2 = checkpoints[checkpoint_index + 2]

    max_distance = 2 ** 30
    best_m = max_distance
    best_distance = max_distance
    best_delta_angle = 0
    best_a = 200

    collision_radius = 300
    iterations = 15

    stage0 = [x, y, vx, vy, angle]

    p = A([x, y])
    v = A([vx, vy])
    rad = math.radians(angle)

    stage0 = (p, v, rad)

    for delta_angle, a in SEARCH_COS2:
        stage = stage0
        stages = [stage]

        for i in range(iterations):
            stage = fast_next_move(*stage, a, delta_angle)
            stages.append(stage)

        pp = [stage[0] for stage in stages]
        ppp = list(zip(pp[:-1], pp[1:]))
        # print(ppp)

        d0 = [distance_point_to_line(t0, a, b) for a, b in ppp]
        d1 = [distance_point_to_line(t1, a, b) for a, b in ppp]
        d2 = [distance_point_to_line(t2, a, b) for a, b in ppp]

        pr0 = [x < collision_radius for x in d0] + [True]
        pr1 = [x < collision_radius for x in d1] + [True, True]
        pr2 = [x < collision_radius for x in d1] + [True, True, True]

        distance = min(d0)

        m0 = pr0.index(True)
        m1 = pr1.index(True, m0)
        m2 = pr2.index(True, m1)

        if m2 < best_m:
            best_m = m2
            best_delta_angle, best_a = delta_angle, a
        elif best_distance > distance:
            best_distance = distance
            best_delta_angle, best_a = delta_angle, a

    return round(math.degrees(best_delta_angle)), best_a


def get_closest_next_step(x, y, vx, vy, angle, checkpoint_index):
    print(x, y, vx, vy, angle, checkpoint_index, file=sys.stderr)
    # best_delta_angle = 0
    # best_a = 200
    # target = checkpoints[checkpoint_index]
    # next_target = checkpoints[(checkpoint_index + 1)%n]
    # next_target_turn = next_target - target
    # p = A([x, y])
    # v = A([vx, vy])
    # best_distance = dot(target - p - v, target - p - v)

    # next_target_turn_distance = dot(next_target_turn, next_target_turn)

    delta_angle, a = find_best_dist(x, y, vx, vy, angle, checkpoint_index)

    # if best_distance < 600**2 and next_target_turn_distance < 2000**2:
    #     a = 30

    # if best_distance < 1500**2 and next_target_turn_distance < 1000**2:
    #     a = 5

    return delta_angle, a


while True:
    checkpoint_index, x, y, vx, vy, angle = [int(i) for i in input().split()]
    delta_angle, a = get_closest_next_step(x, y, vx, vy, angle, checkpoint_index)
    print(f"EXPERT {delta_angle} {a} ->{checkpoint_index}")
