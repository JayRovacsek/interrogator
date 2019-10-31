#!/usr/bin/env python3
import requests
import os
from multiprocessing import Pool

def main():
    urls = []
    for x in range(1,32):
        for y in range(0,24):
            urls.append(f'https://data.gharchive.org/2015-01-{x}-{y}.json.gz')
    try:
        pool = Pool()
        pool.map(download,urls)
    finally:
        pool.close()
        pool.join()

def download(url):
    filename = url.split('/')[-1]
    print(f'Processing: {filename}')
    if not os.path.exists(f'../test_files/{filename}'):
        try:
            r = requests.get(url, allow_redirects=True)
            with open(url.split('/')[-1], 'wb') as f:
                f.write(r.content)
        except Exception as exception:
            print(f'Encountered an error:\n{exception}\nWas processing: {filename}')
        finally:
            pass
    else:
        print(f'File already exists: {filename}')

if __name__ == '__main__':
    main()
