import argparse
from multiprocessing import Pool, Process, Pipe, cpu_count

from lib_try_key import try_key

#test_telegram = "3E44A5112655687276077A39003005EBAEEB906AE817B45D3CC6B46A955BAD34DEA47B00F860ACBB6D280069A227B334CEE23006878125BBAD10EDADAD9635"
test_telegram = "3E44A5112655687276077A6ED330055C1B9AB21431F33C04BBE4741DCE827E6849F8407FFCDFB7EFB0262CC350CE8AD13A7B2DE5BE281C5896B6D4E06FDC3A"
#key_suffix = "10E66D83F8"
#key_space = 16**6
#key_space = 10000
#waypoints_every = key_space // 1000
printworthy_results = { "values" }


def key_range(key_suffix, start, end):
    prefix_length = 16 - len(key_suffix)
    for key_prefix in range(start, end):
       yield f"{key_prefix:0{prefix_length}x}{key_suffix}{key_prefix:0{prefix_length}x}{key_suffix}"


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--processes", help="the number of processes", type=int, default=cpu_count())
    parser.add_argument("--key-start", help="the start of the keyspace", type=int, default=0)
    parser.add_argument("--key-end", help="the end of the keyspace", type=int, default=16**6)
    parser.add_argument("--key-suffix", help="the end of the keyspace", type=str, default="10E66D83F8")
    parser.add_argument("--telegram", help="the telegram to test with", type=str, default=test_telegram)
    parser.add_argument("--meter-driver", help="the meter driver", type=str, default="hydrus")
    parser.add_argument("--meter-id", help="the meter id", type=str, default="72685526")
    parser.add_argument("--waypoints", help="the number of waypoints to print", type=int, default=1000)

    args = parser.parse_args()

    num_keys = args.key_end - args.key_start
    waypoints_every = num_keys // args.waypoints

    with Pool(processes=args.processes, maxtasksperchild=100000) as pool:
        results = pool.imap_unordered(
            try_key,
            ( (key, args.meter_driver, args.meter_id, args.telegram)
                for key in key_range(args.key_suffix, args.key_start, args.key_end) ),
        )
        pool.close()
        for index, result in enumerate(results):
            if result[0] in printworthy_results:
                print(result)
            elif index > 0 and index % waypoints_every == 0:
                print(("progress", result[1], index))
        pool.join()
