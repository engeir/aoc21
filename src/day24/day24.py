import sys

def read_input():
    lines = [line.strip("\n").split() for line in sys.stdin.readlines()]
    return lines

def sort_inps():
    d = {}
    for i in range(14):
        d[i] = []
    c = -1
    for l_ in read_input():
        if len(l_) == 2:
            c += 1
        d[c].append(l_)
    return d

def the_alu():
    inp_range = sort_inps()
    inps = [9 for _ in range(14)]
    while inps != [1 for _ in range(14)]:
        if inps[1:] == [1 for _ in range(13)]:
            inps[0] -= 1
        print("new")
        print(inps)
        d = {"w": 0, "x": 0, "y": 0, "z": 0}
        for idx, val in enumerate(inps):
            rules = inp_range[idx]
            d = evaluate_rules(rules, val, d)
            if d["z"] > 10**7:
                print(1, inps[idx])
                if inps[idx] > 1:
                    inps[idx] -= 1
                    break
                elif inps[idx - 1] > 1:
                    inps[idx - 1] -= 1
                    break
    # for val in reversed(range(1, 10)):
    #     for idx in range(14):
    #         rules = inp_range[idx]
    #         d = evaluate_rules(rules, val, d)
    #         if d["z"] > 10**20:
    #             d = {"w": 0, "x": 0, "y": 0, "z": 0}
    #             break
    #     # break
    #     if d["z"] == 0:
    #         print("Success")
    #         print(d)
    #         break
    #     print(d)
    # for idx in range(14):
    #     d = {"w": 0, "x": 0, "y": 0, "z": 0}
    #     for val in reversed(range(1, 10)):
    #         d = evaluate_rules(inp_range[idx], val, d)
    #     if idx == 0:
    #         break
    # for monad in reversed(range(11111111111111, 99999999999999)):
    #     d = {"w": 0, "x": 0, "y": 0, "z": 0}
    #     for idx, ms in enumerate(str(monad)):
    #         rules = inp_range[idx]
    #         d = evaluate_rules(rules, ms, d)
    #         # print(d)
    #         if d["z"] > 10**2:
    #             break
    #     # break
    #     if d["z"] == 0:
    #         break

def evaluate_rules(rules, input, d):
    d[rules[0][-1]] = int(input)
    for r in rules[1:]:
        val = d[r[2]] if r[2] in d else int(r[2])
        if r[0] == "mul":
            d[r[1]] *= val
        elif r[0] == "add":
            d[r[1]] += val
        elif r[0] == "div":
            keep = d[r[1]] / val
            d[r[1]] = int(keep)
        elif r[0] == "mod":
            d[r[1]] %= val
        elif r[0] == "eql":
            d[r[1]] = 1 if d[r[1]] == val else 0
        else:
            sys.exit("You did something wrong")
    # if d["z"] == 0:
    #     print("Success")
    return d


if __name__ == "__main__":
    # read_input()
    the_alu()
