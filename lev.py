#!/usr/bin/env python3

#
#
# https://gist.github.com/rexim/2bd6b1b6c3de95a3cf6d00b172ca98bb
#
#

import functools

@functools.lru_cache(maxsize=None)
def lev_ref(s1, s2):
    if len(s1) == 0:
        return len(s2)
    if len(s2) == 0:
        return len(s1)
    if s1[-1] == s2[-1]:
        return lev_ref(s1[:-1], s2[:-1])  # ignore
    return 1 + min(
        lev_ref(s1[:-1], s2),  # remove
        lev_ref(s1, s2[:-1]),  # add
        lev_ref(s1[:-1], s2[:-1]),
    )  # replace


TRACE = True

if TRACE:

    def trace_cache(cache, actions):
        for row in range(len(cache)):
            for col in range(len(cache[row])):
                item = cache[row][col]
                action = actions[row][col]
                print(f"{item} ({action})".ljust(6), end=" ")
            print()
        print()

else:

    def trace_cache(*args):
        pass


IGNORE = "I"
ADD = "A"
REMOVE = "R"
SUBST = "S"


def lev(s1, s2):
    cache = []
    actions = []

    for _ in range(len(s1) + 1):
        cache.append(["-"] * (len(s2) + 1))
        actions.append(["-"] * (len(s2) + 1))

    cache[0][0] = 0
    actions[0][0] = IGNORE
    trace_cache(cache, actions)

    for n2 in range(1, len(s2) + 1):
        n1 = 0
        cache[n1][n2] = n2
        actions[n1][n2] = ADD
        trace_cache(cache, actions)

    for n1 in range(1, len(s1) + 1):
        n2 = 0
        cache[n1][n2] = n1
        actions[n1][n2] = REMOVE
        trace_cache(cache, actions)

    for n1 in range(1, len(s1) + 1):
        for n2 in range(1, len(s2) + 1):
            if s1[n1 - 1] == s2[n2 - 1]:
                cache[n1][n2] = cache[n1 - 1][n2 - 1]
                actions[n1][n2] = IGNORE
                trace_cache(cache, actions)
                continue  # ignore

            remove = cache[n1 - 1][n2]
            add = cache[n1][n2 - 1]
            subst = cache[n1 - 1][n2 - 1]

            cache[n1][n2] = remove
            actions[n1][n2] = REMOVE

            if cache[n1][n2] > add:
                cache[n1][n2] = add
                actions[n1][n2] = ADD

            if cache[n1][n2] > subst:
                cache[n1][n2] = subst
                actions[n1][n2] = SUBST

            cache[n1][n2] += 1

            trace_cache(cache, actions)

    trace = []
    n1 = len(s1)
    n2 = len(s2)
    while n1 > 0 or n2 > 0:
        action = actions[n1][n2]
        if action == ADD:
            n2 -= 1
            trace.append((ADD, s2[n2]))
        elif action == REMOVE:
            n1 -= 1
            trace.append((REMOVE, s1[n1]))
        elif action == IGNORE:
            n1 -= 1
            n2 -= 1
            trace.append((IGNORE, s1[n1]))
        elif action == SUBST:
            n1 -= 1
            n2 -= 1
            trace.append((SUBST, s1[n1], s2[n2]))
        else:
            assert False, "unreachable"
    print(list(reversed(trace)))

    return cache[n1][n2]


# print(lev_ref("adddfjksdfkdgjks", "addf9sdfjksdfjkljsdf"))
# print(lev("adddfjksdfkdgjks", "addf9sdfjksdfjkljsdf"))
print(lev("add", "dady"))
# print(lev("foo", "foooooooo"))
# print(lev("", "foo"))
