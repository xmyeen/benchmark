#!/usr/bin/env python
# -*- coding:utf-8 -*-

import os, sys, warnings, getopt, tempfile, asyncio, timeit
import aiofiles

MB_DEF, GB_DEF = 2 ** 20, 2 ** 30

def write_sync(file_path:str,  step:int, total:int):
    data = b'\0' * step
    with open(file_path, mode='wb') as w:
        for _ in range(0, total, step):
            w.write(data)

async def write_async(file_path:str, step:int, total:int):
    data = b'\0' * step
    async with aiofiles.open(file_path, mode='wb') as w:
        for _ in range(0, total, step):
            await w.write(data)

if '__main__' == __name__:
    # if 2 != len(sys.argv):
    #     warnings.warn("Miss work directory. Please specify it")
    #     sys.exit(1)

    out_dir = os.getcwd()
    batch_bytes, total_bytes, repeat_num = 1024, 100 * MB_DEF, 3
    opts,args = getopt.getopt(sys.argv[1:], "t:d:", ["times=", "bb=", "tb=", "dir="])
    for name, value in opts:
        if name in ("-t", "--times"):
            repeat_num = int(value)
        elif name in ("--bb"):
            batch_bytes = int(value)
        elif name in ("--tb"):
            total_bytes = int(value)
        elif name in ("-d", "--dir"):
            out_dir = value

    if not os.path.exists(out_dir): os.makedirs(out_dir)
    print("Batch-Bytes", ":", batch_bytes)
    print("Total-Bytes", ":", total_bytes)
    print("Times", ":", repeat_num)

    with tempfile.TemporaryDirectory(dir = out_dir) as td:
        print("Output-Directory", ":", out_dir)
        print("Syn-Elapsed", ":", timeit.timeit(lambda: write_sync(os.path.join(td, "file_1"), batch_bytes, total_bytes), number = repeat_num) / repeat_num)
        print("Asyn-Elapsed", ":", timeit.timeit(lambda: asyncio.run(write_async(os.path.join(td, "file_2"), batch_bytes, total_bytes)), number = repeat_num) / repeat_num)