import argparse
from multiprocessing import Pool, Process, Pipe, cpu_count

from lib_try_key import try_key

#test_telegram = "3E44A5112655687276077A39003005EBAEEB906AE817B45D3CC6B46A955BAD34DEA47B00F860ACBB6D280069A227B334CEE23006878125BBAD10EDADAD9635"
test_telegram = "3E44A5112655687276077A6ED330055C1B9AB21431F33C04BBE4741DCE827E6849F8407FFCDFB7EFB0262CC350CE8AD13A7B2DE5BE281C5896B6D4E06FDC3A"

def read_keys(filename):
    with open(filename, 'rt') as file:
        return [ line.strip() for line in file ]


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("keyfile", help="the file containing a key per line", type=str)
    parser.add_argument("--processes", help="the number of processes", type=int, default=cpu_count())
    parser.add_argument("--telegram", help="the telegram to test with", type=str, default=test_telegram)
    parser.add_argument("--meter-driver", help="the meter driver", type=str, default="hydrus")
    parser.add_argument("--meter-id", help="the meter id", type=str, default="72685526")

    args = parser.parse_args()

    keys = read_keys(args.keyfile)
    num_keys = len(keys)

    print(f"Trying {num_keys} keys")

    with Pool(processes=args.processes, maxtasksperchild=100000) as pool:
        results = pool.imap_unordered(
            try_key,
            ( (key, args.meter_driver, args.meter_id, args.telegram) for key in keys ),
        )
        pool.close()
        for index, result in enumerate(results):
            if len(result[2]) > 0:
                print(result)
            else:
                print(("progress", result[1]))
        pool.join()
