#!/bin/python

# Not the most elegant piece of code you'll ever read. That's for sure :(
if __name__ == "__main__":
    import sys

    keys = []
    needed_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    validated_keys = {k: False for k in needed_keys}
    c = 0

    for line in sys.stdin:
        line = line.strip("\n")
        if len(line) == 0:
            print(validated_keys)
            if all(validated_keys.values()):
                c += 1
            validated_keys = {k: False for k in needed_keys}
        else:
            for kv in line.split(" "):
                tt = kv.split(":")
                k, v = tt[0], tt[1]
                if k == "byr":
                    try:
                        if 1920 <= int(v) <= 2002 and len(v) == 4:
                            validated_keys[k] = True
                    except:
                        pass
                elif k == "iyr":
                    try:
                        if 2010 <= int(v) <= 2020 and len(v) == 4:
                            validated_keys[k] = True
                    except:
                        pass
                elif k == "eyr":
                    try:
                        if 2020 <= int(v) <= 2030 and len(v) == 4:
                            validated_keys[k] = True
                    except:
                        pass
                elif k == "hgt":
                    try:
                        if v.endswith("cm"):
                            if 150 <= int(v[:-2]) <= 193:
                                validated_keys[k] = True
                        elif v.endswith("in"):
                            if 59 <= int(v[:-2]) <= 76:
                                validated_keys[k] = True
                    except:
                        pass
                elif k == "hcl":
                    if v.startswith("#"):
                        if v[1:].strip("0123456789abcdef") == "":
                            if len(v) == 7:
                                validated_keys[k] = True
                elif k == "ecl":
                    if v in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]:
                        validated_keys[k] = True
                elif k == "pid":
                    if v.strip("0123456789") == "":
                        if len(v) == 9:
                            validated_keys[k] = True
    print(c)
