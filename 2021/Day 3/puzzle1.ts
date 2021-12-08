import input from "./input.js"

let gamma = ""
let epsilon = ""

let mostCommonBit: number
let leastCommonBit : number

function setBits(most: number, least: number) {
    mostCommonBit = most
    leastCommonBit = least
}

for (let i = 0; i < input[0].split("").length; i++) {
    let ones = 0
    let zeros = 0

    for (let k = 0; k < input.length; k++) {
        let bit = parseInt(input[k].split("")[i])
        bit === 0 ? zeros++ : ones++
    }

    ones > zeros ? setBits(1,0) : setBits(0,1)

    gamma = `${gamma}${mostCommonBit}`
    epsilon = `${epsilon}${leastCommonBit}`
}

const gammaRaw = parseInt(gamma, 2)
const epsilonRaw = parseInt(epsilon, 2)

console.log(gammaRaw * epsilonRaw)
