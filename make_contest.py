#!/usr/bin/env python3
import argparse
import shutil
import time
from pathlib import Path


CURRENT_CONTEST_LINK='current_contest'
TEMPLATES_DIR='_templates'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("count", type=int,
                        help="number of problems")
    parser.add_argument("path",
                        help="path to contest")
    args = parser.parse_args()

    pwd = Path('.')
    templates_dir = pwd / TEMPLATES_DIR
    contest_dir = pwd / args.path
    contest_dir.resolve()

    if not templates_dir.exists():
        print(f'Templates dir {TEMPLATES_DIR} not found')

    if not contest_dir.exists():
        print(f'Create {contest_dir}')
        contest_dir.mkdir(parents=True)

    for i in range(args.count):
        problem_dir = contest_dir / f'{i + 1}'
        if problem_dir.exists():
            continue

        print(f'Create {problem_dir}')
        problem_dir.mkdir(parents=True)

        for template in templates_dir.iterdir():
            if template.suffix:
                destination = problem_dir / f'{i + 1}{template.suffix}'
                print(f'{destination}')
                shutil.copy(template, destination)

        in_file = problem_dir / f'{i + 1}.in'
        with open(in_file, 'w') as f:
            f.write('')

    contest_dir_link = pwd / CURRENT_CONTEST_LINK

    if contest_dir_link.exists():
        new_name = f'{CURRENT_CONTEST_LINK}_{time.time_ns()}'
        print(f'contest path link exists: mv -> {new_name}')
        contest_dir_link.rename(new_name)

    print(f'Create symlink {contest_dir_link} -> {contest_dir}')
    contest_dir_link.symlink_to(contest_dir)

    print(*sorted(contest_dir.iterdir()), sep='\n')


if __name__ == '__main__':
    main()
