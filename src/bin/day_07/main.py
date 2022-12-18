import sys

def make_fs(instructions):
    root = {'path': '', 'children': {}, 'parent': None, 'type': 'dir'}
    path_segments = ['']
    current_level = root

    for line in instructions:
        op, arg, *rest = line.split()
        if op == '$':
            if arg == 'ls':
                continue
            elif arg == 'cd':
                target = rest[0]
                if target == '..':
                    path_segments.pop()
                    current_level = current_level['parent']
                elif target == '/':
                    path_segments = ['']
                    current_level = root
                else:
                    path_segments.append(target)
                    current_level = current_level['children'][target]
        elif op == 'dir':
            current_level['children'][arg] = {'path': arg, 'children': {}, 'parent': current_level, 'type': 'dir'}
        else:
            current_level['children'][arg] = {'path': arg, 'size': int(op), 'parent': current_level, 'type': 'file'}
    return root


def traverse_from_bottom(fs, fn):
    for child in fs.get('children', {}).values():
        traverse_from_bottom(child, fn)
    fn(fs)


def traverse_from_top(fs, fn):
    fn(fs)
    for child in fs.get('children', {}).values():
        traverse_from_top(child, fn)


def annotate_with_size(fs):
    def calc_size(fs):
        if 'children' in fs:
            fs['size'] = sum(child['size'] for child in fs['children'].values())

    traverse_from_bottom(fs, calc_size)


def annotate_with_abspath(fs):
    def calc_abspath(fs):
        if fs['parent']:
            fs['abspath'] = fs['parent']['path'] + '/' + fs['path']

    traverse_from_top(fs, calc_abspath)


def dirs_by_size(fs):
    dirs_by_size = {'/': fs['size']}

    def check_dir(fs):
        if fs['type'] == 'dir':
            if fs['parent']:
                dirs_by_size[fs['abspath']] = fs['size']

    traverse_from_top(fs, check_dir)
    return dirs_by_size

if __name__ == '__main__':
    fs = make_fs(sys.stdin)
    annotate_with_size(fs)
    annotate_with_abspath(fs)
    dirs = dirs_by_size(fs)
    part_1 = sum(size for dir, size in dirs.items() if size <= 100000)
    print(f'Part 1: {part_1}')
    capacity = 70000000
    required = 30000000
    free = capacity - fs['size']
    part_2 = min(size for dir, size in dirs.items() if size + free >= required)
    print(f'Part 2: {part_2}')