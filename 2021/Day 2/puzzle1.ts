import input from './input.js'

type Direction = 'forward' | 'down' | 'up'

let depth = 0
let horizontalPos = 0

function goForward(change: number) {
    horizontalPos += change
}

function goDown(change: number) {
    depth += change
}

function goUp(change: number) {
    depth -= change
}

for (let i = 0; i < input.length; i++) {
    let command = input[i].split(" ")
    let commandtDirection = command[0] as Direction
    let commandValue = +command[1]

    switch (commandtDirection) {
        case 'forward':
            goForward(commandValue)
            break
        case 'down':
            goDown(commandValue)
            break
        case 'up':
            goUp(commandValue)
            break
    }
}

console.log(depth * horizontalPos)