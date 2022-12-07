const fs = require('fs');
const { once } = require('node:events');
const readline = require('readline');
const path = require('path');

const TOTAL_SPACE = 70000000;
const UPDATE_SIZE = 30000000;

const readLines = (inputFile) => {
    const fileStream = fs.createReadStream(inputFile);
    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    });
    return rl;
}

const readFileTree = async () => {
    let rootDir = {
        name: '/',
        size: 0,
        children: [],
        parent: null
    };

    let currentDir = rootDir;
    let isListing = false;

    const changeDirectory = (dirName) => {
        switch (dirName) {
            case '/': {
                currentDir = rootDir;
                break;
            }
            case '..': {
                currentDir = currentDir.parent;
                break;
            }
            default: {
                let child = currentDir.children.find(c => c.name === dirName);
                currentDir = child;
            }
        }
    };

    const addListing = (size, name) => {
        size = Number.parseInt(size) || 0;
        currentDir.children.push({
            name,
            size,
            children: [],
            parent: currentDir
        });
    };

    const rl = readLines(path.resolve(__dirname, "day7_input"));
    rl.on('line', (line) => {
        let parts = line.split(' ');
        if (parts[0] === '$') {
            // process command
            isListing = false;
            let commandName = parts[1];
            switch (commandName) {
                case 'cd': {
                    let dirName = parts[2];
                    changeDirectory(dirName);
                    break;
                }
                case 'ls': {
                    isListing = true;
                    break;
                }
            }
        } else if (isListing) {
            // listing directories inside the current directory
            addListing(parts[0], parts[1]);
        } else {
            throw new Error('invalid input');
        }
    });

    await once(rl, 'close');

    return rootDir;
}

const getDirSizes = (node, sizes) => {
    let children = node.children;
    if (children.length === 0) {
        return node.size;
    }

    let size = node.children.reduce((acc, n) => acc + getDirSizes(n, sizes), node.size);
    sizes.push(size);
    return size;
}

const problem1 = async () => {
    let rootNode = await readFileTree();
    let sizes = [];
    getDirSizes(rootNode, sizes);
    let total = sizes.filter(s => s < 100000).reduce((acc, s) => acc + s, 0);
    console.log(total);
}

const problem2 = async () => {
    let rootNode = await readFileTree();
    let sizes = [];
    let totalUsedSpace = getDirSizes(rootNode, sizes);
    let remainingSpace = TOTAL_SPACE - totalUsedSpace;
    let spaceRequired = UPDATE_SIZE - remainingSpace;

    sizes.sort();
    let sizeToDelete = sizes.find((s) => s > spaceRequired);
    console.log(sizeToDelete);
}

problem1();
problem2();
