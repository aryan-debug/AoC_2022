from __future__ import annotations


class File:
    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size

    def __str__(self) -> str:
        return f"Name: {self.name}\nSize: {self.size}"


class Directory:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.dirs = {}
        self.files = {}

    def add_child(self, child: str):
        if "dir" in child:
            self.add_dir(child)
        else:
            self.add_file(child)

    def add_dir(self, info):
        dir_name = info.split()[-1]
        self.dirs[dir_name] = Directory(dir_name, self)

    def add_file(self, info):
        file_size, file_name = info.split()
        self.files[file_name] = File(file_name, int(file_size))

    def get_directory(self, line: str) -> Directory:
        dir_name = line.split()[-1]
        return self.dirs.get(dir_name)


def read_file():
    with open("input.txt") as file:
        content = [line.strip() for line in file.readlines()]

    return content


def create_tree(terminal_io):
    root = Directory("/", None)
    current_directory = root
    add_children = False
    for line in terminal_io[1:]:
        if "$ cd" in line:
            add_children = False
            if ".." in line:
                current_directory = current_directory.parent
            else:
                current_directory = current_directory.get_directory(line)
        if add_children:
            current_directory.add_child(line)
        if line == "$ ls":
            add_children = True

    return root


def get_dir_size(file_tree, total=[]):
    if not file_tree.dirs:
        file_total = 0
        for file_name in file_tree.files:
            file_total += file_tree.files[file_name].size
        total.append(file_total)
        return file_total
    else:
        sum = 0
        for dir_name in file_tree.dirs:
            dir_size = get_dir_size(file_tree.dirs[dir_name])
            if isinstance(dir_size, list):
                sum += dir_size[-1]
            else:
                sum += dir_size
        for file_name in file_tree.files:
            sum += file_tree.files[file_name].size
        total.append(sum)
    return total


def main():
    terminal_io = read_file()
    file_tree = create_tree(terminal_io)
    dirs_size = get_dir_size(file_tree)
    total_space = 70_000_000
    required_unused_space = 30_000_000
    unused_space = total_space - max(dirs_size)
    space_to_delete = required_unused_space - unused_space
    print(min(filter(lambda x: x > space_to_delete, dirs_size)))


main()
