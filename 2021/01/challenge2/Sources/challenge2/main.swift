import Foundation

let currentDir = FileManager().currentDirectoryPath
let filename = CommandLine.arguments[1]
let lines = try String(contentsOfFile: "\(currentDir)/\(filename)").split(whereSeparator: \.isNewline)

let numbers = lines.map{ Int($0)! }
var numberIncreased = 0;
for (index, number) in numbers.enumerated() {
    if index > 2  {
        let firstWindow = numbers[index - 3] + numbers[index - 2] + numbers[index-1]
        let secondWindow = numbers[index - 2] + numbers[index - 1] + numbers[index]
        if secondWindow > firstWindow {
            numberIncreased+=1;
        }
    }
}

print("Total amount of increments: \(numberIncreased)")