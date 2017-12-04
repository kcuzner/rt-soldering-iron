#!/usr/bin/env python3

import argparse, subprocess, sys, os
import json

def get_artifacts(args):
    """
    Gets the rust build artifact paths
    """
    command = ['xargo', 'build', '--message-format=json', '-q']
    if args.release:
        command.append('--release')
    output = subprocess.check_output(command)
    for line in output.splitlines():
        message = json.loads(line)
        if 'reason' in message and message['reason'] == 'compiler-artifact' and\
            message['target']['name'] in args.targets:
            for f in message['filenames']:
                yield f


def objdump(args):
    """
    Sub-command function for performing an object dump
    """
    for a in get_artifacts(args):
        with open(os.path.splitext(a)[0] + '.lst', 'w') as f:
            print('Dumping ' + a)
            subprocess.check_call([args.objdump, '-D', a], stdout=f)

def objcopy(args):
    """
    Sub-command function for performing an object copy
    """
    ext = '.bin' if args.type == 'binary' else\
            '.hex'
    for a in get_artifacts(args):
        print('Copying ' + a + ' to ' + args.type)
        command = [args.objcopy, '-O', args.type, a, os.path.splitext(a)[0]+ext]
        subprocess.check_call(command)

def binary_size(args):
    """
    Sub-command function for outputting size
    """
    for a in get_artifacts(args):
        command = [args.size, '--format='+args.format, a]
        p = subprocess.Popen(command, stdout=subprocess.PIPE)
        print('Sizing ' + a)
        for line in iter(p.stdout.readline, b''):
            print(line.decode('ascii').rstrip())
        p.wait()

def main():
    parser = argparse.ArgumentParser(description='Performs embedded-device specific operations on rust build artifacts.')
    parser.add_argument('--release', action='store_true',
            help='Execute on the release build')
    parser.add_argument('--objcopy', default='arm-none-eabi-objcopy',
            help='Path to the objcopy executable')
    parser.add_argument('--target', metavar='TARGET', action='append', dest='targets',
            help='Target name for compiler artifacts')
    parser.add_argument('--cargo', default='xargo',
            help='Path to the cargo executable')
    subparsers = parser.add_subparsers()
    subparsers.required = True
    subparsers.dest = 'command'

    parser_dump = subparsers.add_parser('dump', help='Creates a dump disassembly')
    parser_dump.add_argument('--objdump', default='arm-none-eabi-objdump',
            help='Path to the objdump executable')
    parser_dump.set_defaults(func=objdump)

    parser_bin = subparsers.add_parser('copy', help='Creates binary and hex files')
    parser_bin.add_argument('type', choices=['binary', 'ihex'])
    parser_bin.set_defaults(func=objcopy)

    parser_size = subparsers.add_parser('size', help='Outputs the resulting sizes of the elf sections')
    parser_size.add_argument('--size', default='arm-none-eabi-size',
            help='Path to the size executable')
    parser_size.add_argument('--format', default='SysV',
            help='Format argument for the size executable')
    parser_size.set_defaults(func=binary_size)

    parser_flash = subparsers.add_parser('flash', help='Flashes the microcontroller')
    parser_flash.add_argument('--openocd', default='openocd',
            help='Path to the openocd executable')

    args = parser.parse_args()
    args.func(args)

if __name__ == '__main__':
    main()

