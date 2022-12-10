# Correct Python prototype for Day 7 used to validate my "proper" Rust solution.
MAX = 100000

def ptree(root, depth=0):
    print(" " * depth, " - ", root[0] if len(root[0]) else "/", root[1] if root[1] > 0 else "(dir)")
    for k, v in root[2].items():
        ptree(v, depth + 1)


def sz(node):
    if node[1] > 0:
        assert len(node[2]) == 0, "Found unexpected file with children: {}!!".format(node)
        return node[1]
    else:
        return sum([sz(v) for k, v in node[2].items()])


def gather(node):
    if len(node[2]) == 0:
        return 0

    nn = sz(node)
    if nn > MAX:
        nn = 0

    for k, v in node[2].items():
        nn += gather(v)

    return nn


def main():
    in_fpath = "input/07.txt"

    root = ("", 0, dict(), None)
    cur = root

    with open(in_fpath, "r") as in_f:
        for row in in_f.readlines():
            row = row.strip()
            p = row.split(" ")
            if p[0] == "$":
                if p[1] == "cd":
                    if p[2] == "..":
                        cur = cur[3]
                    elif p[2] == "/":
                        cur = root
                    else:
                        if p[2] not in cur[2]:
                            cur[2][p[2]] = (p[2], 0, dict(), cur)
                        cur = cur[2][p[2]]
                elif p[1] == "ls":
                    pass
                else:
                    raise ValueError()
            else:
                if p[0] == "dir":
                    if p[1] not in cur[2]:
                        cur[2][p[1]] = (p[1], 0, dict(), cur)
                    else:
                        print("WARN: dir ls'd twice")
                else:
                    cur[2][p[1]] = (p[1], int(p[0]), dict(), cur)

    ptree(root)
    print(gather(root))


if __name__ == "__main__":
    main()