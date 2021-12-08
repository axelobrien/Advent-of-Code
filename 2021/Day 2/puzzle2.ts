import input from './input.js'

type Direction = 'forward' | 'up' | 'down'

let depth = 0
let horizontalPos = 0
let aim = 0

function goForward(change: number) {
    horizontalPos += change
    depth += (change * aim)
}

function goDown(change: number) {
    aim += change
}

function goUp(change: number) {
    aim -= change
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