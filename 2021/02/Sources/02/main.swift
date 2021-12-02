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
    let instructions = toInstructions(lines: lines)
    var depth = 0
    var position = 0
    for instruction in instructions {
        switch instruction.direction {
        case .up:
            depth -= instruction.steps
        case .down:
            depth += instruction.steps
        case .forward:
            position += instruction.steps
        default:
            print("lol")
        }
    }
    print("Position: \(position), Depth: \(depth), total: \(position * depth)")
}

func challenge2(lines: [String]) {
    let instructions = toInstructions(lines: lines)
    var depth = 0
    var position = 0
    var aim = 0
    for instruction in instructions {
        switch instruction.direction {
        case .up:
            aim -= instruction.steps
        case .down:
            aim += instruction.steps
        case .forward:
            position += instruction.steps
            depth += instruction.steps * aim
        default:
            print("lol")
        }
    }
    print("Position: \(position), Depth: \(depth), Aim: \(aim), total: \(position * depth)")
}

func toInstructions(lines: [String]) -> [Instruction] {
    lines.map {
        line -> Instruction in
        let dir_steps = line.split(separator: " ", maxSplits: 1)
        var dir: Direction
        switch dir_steps[0] {
        case "up":
            dir = .up
        case "forward":
            dir = .forward
        case "down":
            dir = .down
        default:
            dir = .none
        }
        return Instruction(direction: dir, steps: Int(dir_steps[1]) ?? 0)
    }
}
