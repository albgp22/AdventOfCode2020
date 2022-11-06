
import sys

def get_seat_id(partitioning):
    bin_row_st = partitioning[:-3].replace("F","0").replace("B","1")
    bin_seat_st = partitioning[-3:].replace("L","0").replace("R","1")
    index = int(bin_row_st,2)*8+int(bin_seat_st,2)
    return index


if __name__ == "__main__":
    ids = [get_seat_id(line.strip("\n").strip(" ")) for line in sys.stdin]
    all_missing_seats=[i for i in range(0,127*8) if i not in ids]
    all_missing_seats=[s for s in all_missing_seats if 20<=s//8<=110]
    print(all_missing_seats)
        

