import os, sys, json, subprocess, time;
from types import *;
from typing import *;
from dataclasses import dataclass
from argparse_dataclass import dataclass as argparser
from enum import IntEnum

@dataclass
class Process:
    proc: subprocess.Popen[bytes]
    name: str
    url: str
    file: str
    genre: str

PROCESSES: List[Process] =[]

class Level(IntEnum):
    ERROR = 0
    WARN = 1
    INFO = 2
    DEBUG = 3
        
def level_to_prefix(l: Level) -> str:
    match l:
        case Level.ERROR:
            return "\033[0;31m\033[1merror\033[0m"
        case Level.WARN:
            return "\033[1;33m\033[1mwarn\033[0m"
        case Level.INFO:
            return "\033[1;32m\033[1minfo\033[0m"
        case Level.DEBUG:
            return "\033[1;34m\033[1mdebug\033[0m"
            
        

class Logger:
    level: Level = Level.INFO

    def error(self, s: str):
        if self.level >= Level.ERROR:
            print(f"{level_to_prefix(Level.ERROR)}: {s}")
    def warn(self, s: str):
        if self.level >= Level.WARN:
            print(f"{level_to_prefix(Level.WARN)}: {s}")
    def info(self, s: str):
        if self.level >= Level.INFO:
            print(f"{level_to_prefix(Level.INFO)}: {s}")
    def debug(self, s: str):
        if self.level >= Level.DEBUG:
            print(f"{level_to_prefix(Level.DEBUG)}: {s}")

LOGGER = Logger()


if os.name == 'nt':
    YTDLP="yt-dlp.exe"
else:
    YTDLP="yt-dlp"

@dataclass
class ManifestSong:
    name: str
    url: str

type ManifestGenre = List[ManifestSong]

type Manifest = Dict[str, ManifestGenre]

@argparser
class CliArgs:
    manifest: str = "./manifest.json"
    output_dir: str = "./"
    debug: bool = False

def main(cliargs: CliArgs) -> int:
    if cliargs.debug:
        LOGGER.level = Level.DEBUG
    manifest: Manifest = get_manifest(cliargs)
    count = 0
    for (genre, v) in manifest.items():
        for song in v:
            if len(PROCESSES) > 10:
                wait_for_procs(10)
            if download_song(genre, song["name"], song["url"]):
                count += 1
    wait_for_procs()
    LOGGER.info(f"Downloaded {count} new songs")


def download_song(dir: str, name: str, url: str) -> bool:
    if not os.path.isdir(dir):
        os.mkdir(dir)

    outfile=f"{dir}/{name}.m4a"

    if os.path.isfile(outfile):
        LOGGER.debug(f"Already downloaded {outfile} ({url}), skipping")
        return False

    LOGGER.info(f"Downloading {outfile} ({url})")

    proc = subprocess.Popen(
        [YTDLP, "-x", "--audio-format", "m4a","-o", outfile, url],
        stdout=open(os.devnull, 'wb')
    )

    PROCESSES.append(Process(proc, name, url, outfile, dir))
    return True


def wait_for_procs(until: int = 0):
    while len(PROCESSES) > until:
        for proc in PROCESSES:
            retcode = proc.proc.poll()
            if retcode is not None: # Process finished.
                PROCESSES.remove(proc)
                LOGGER.info(f"Finished downloading {proc.file}")
                break
            else: # No process is done, wait a bit and check again.
                time.sleep(.1)
                continue


def get_manifest(cliargs: CliArgs) -> Manifest:
    with open(cliargs.manifest, "r") as f:
        data = f.read()
        return json.loads(data)


if __name__ == "__main__":
    ret = main(CliArgs.parse_args(sys.argv[1:]))
    sys.exit(ret)