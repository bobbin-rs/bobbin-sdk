import sys, os, struct

def main(args):
    input = sys.stdin
    for line in input:
        row = line.split(' ')
        for r in row:
            v = int(r, 10)
            p = struct.pack('@h', v)
            sys.stdout.buffer.write(p)
            sys.stdout.buffer.write(p)

if __name__ == '__main__':
    main(sys.argv[1:])
