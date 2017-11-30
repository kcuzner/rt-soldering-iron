#!/usr/bin/env python3

# Runs a command on the compiler artifacts for some target.
#
# Kevin Cuzner

import os, sys, argparse, subprocess
import json

def main():
    parser = argparse.ArgumentParser(description='Invokes commands on build artifacts. Pipe the output of cargo with --message-format=json to this script.')
    parser.add_argument('command', type=str, help='Command to execute')
    parser.add_argument('args', type=str, nargs=argparse.REMAINDER, help='Arguments for the command')
    parser.add_argument('--target', metavar='TARGET', action='append', dest='targets', help='Target to run the command on')
    args = parser.parse_args()

    for line in sys.stdin:
        message = json.loads(line)
        if 'reason' in message and message['reason'] == 'compiler-artifact' and\
            message['target']['name'] in args.targets:
            for f in message['filenames']:
                env = dict(os.environ)
                env['RUST_ARTIFACT'] = f
                subprocess.Popen([args.command]+args.args, env=env).wait()

if __name__ == '__main__':
    main()

