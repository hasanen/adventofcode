import Foundation

enum Direction {
    case up
    case forward
    case down
    case none
}

struct Instruction {
    var direction: Direction
    var steps: Int
}

let currentDir = FileManager().currentDirectoryPath
let filename = CommandLine.arguments[2]
let lines = try String(contentsOfFile: "\(currentDir)/\(filename)").split(whereSeparator: \.isNewline).map { String($0) }
switch CommandLine.arguments[1] {
case "1":
    challenge1(lines: lines)
case "2":
    challenge2(lines: lines)
default:
    print("Giev me challenge number: 1 or 2")
}

func challenge1(lines: [String]) {
    var gamma_rate = ""
    var epsilon_rate = ""
    for i in 1 ... lines[0].count {
        var zeros = 0
        var ones = 0
        for line in lines {
            let num = line[line.index(line.startIndex, offsetBy: i - 1)]

            if num == "1" {
                ones += 1
            } else {
                zeros += 1
            }
        }
        if zeros > ones {
            gamma_rate += "0"
            epsilon_rate += "1"
        } else {
            gamma_rate += "1"
            epsilon_rate += "0"
        }
    }
    let gamma_int = binToUInt(bin: gamma_rate)
    let epsilon_int = binToUInt(bin: epsilon_rate)

    print("Gamma rate is \(gamma_rate) (\(gamma_int))")
    print("Epsilon rate is \(epsilon_rate) (\(epsilon_int))")
    print("Result: \(gamma_int * epsilon_int)")
}

func challenge2(lines _: [String]) {
    let oxygen_rating = filter_binary_strings_to_one(values: lines)
    let oxygen_rating_int = binToUInt(bin: oxygen_rating)
    let c02_scrubber_rating = filter_binary_strings_to_one(values: lines, most_common: false)
    let c02_scrubber_rating_int = binToUInt(bin: c02_scrubber_rating)

    print("Oxygen rating is \(oxygen_rating) (\(oxygen_rating_int))")
    print("CO2 scrubber rating is \(c02_scrubber_rating) (\(c02_scrubber_rating_int))")
    print("Result: \(oxygen_rating_int * c02_scrubber_rating_int)")
}

func binToUInt(bin: String) -> UInt {
    strtoul(bin, nil, 2)
}

func filter_binary_strings_to_one(values: [String], index: Int = 0, most_common: Bool = true) -> String {
    var zeros = 0
    var ones = 0
    for line in values {
        let num = line[line.index(line.startIndex, offsetBy: index)]

        if num == "1" {
            ones += 1
        } else {
            zeros += 1
        }
    }
    let expected_num = (zeros > ones && most_common) || (zeros < ones && !most_common) || (zeros == ones && !most_common) ? "0" : "1"

    let filtered = values.filter { line in
        String(line[line.index(line.startIndex, offsetBy: index)]) == expected_num
    }
    if filtered.count == 1 {
        return filtered[0]
    } else {
        return filter_binary_strings_to_one(values: filtered, index: index + 1, most_common: most_common)
    }
}
