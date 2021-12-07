import input from './input.js'

let previousSum: number
let increasesInDepth = 0

for (let i = 0; i < input.length; i++) {
    let currentSum: number
    let workingNumbers: number[] = []

    for (let j = 0; j < 3; j++) {
        if (input[i + j]) {
            workingNumbers.push(input[i + j])
        }
    }
    currentSum = workingNumbers.reduce((a, b) => a + b, 0) // sums array
    if (currentSum > previousSum) {
        increasesInDepth++
    }
    previousSum = currentSum
}

console.log(increasesInDepth)