import input from './input.js'

let previousDepth: number
let increasesInDepth = 0

for (let i = 0; i < input.length; i++) {
    if (previousDepth) {
        if (input[i] > previousDepth) {
            increasesInDepth++
        }
    }
    previousDepth = input[i]
}

console.log(increasesInDepth)