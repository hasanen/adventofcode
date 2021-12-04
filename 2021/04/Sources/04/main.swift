import Foundation

enum Direction {
    case up
    case forward
    case down
    case none
}

class BingoCell {
    var number: Int
    var hit: Bool

    init(number: Int) {
        self.number = number
        hit = false
    }

    public var description: String { "Number: \(number), Hit: \(hit)" }
}

class BingoBoard {
    var board: [[BingoCell]]
    var bingo_number: Int?

    init(board: [[BingoCell]]) {
        self.board = board
    }

    func mark_number(number: Int) {
        for i in 0 ... board.count - 1 {
            for j in 0 ... board[i].count - 1 {
                if board[i][j].number == number {
                    board[i][j].hit = true
                }
            }
        }
        if has_bingo() {
            bingo_number = number
        }
    }

    func has_bingo() -> Bool {
        bingo_in_row() || bingo_in_column()
    }

    func bingo_in_row() -> Bool {
        for i in 0 ... board.count - 1 {
            if board[i].allSatisfy(\.hit) {
                return true
            }
        }

        return false
    }

    func bingo_in_column() -> Bool {
        for r in 0 ... board[0].count - 1 {
            if (board.map { $0[r] }).allSatisfy(\.hit) {
                return true
            }
        }

        return false
    }

    func sumOfUnhitNumbers() -> Int {
        board.joined().filter { !$0.hit }.map(\.number).reduce(0, +)
    }

    func result() -> Int {
        sumOfUnhitNumbers() * (bingo_number ?? 0)
    }
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
    let (numbers, boards) = parseLinesToNumbersAndBoard(lines: lines)

    let board = boardWithFirstBingo(numbers: numbers, boards: boards)

    if board != nil {
        print("Sum of unhit numbers: \(board!.sumOfUnhitNumbers()), result: \(board!.result())")
    } else {
        print("No boards with bingo")
    }
}

func challenge2(lines: [String]) {
    let (numbers, boards) = parseLinesToNumbersAndBoard(lines: lines)

    let board = boardWithLastBingo(numbers: numbers, boards: boards)
    if board != nil {
        print("Sum of unhit numbers: \(board!.sumOfUnhitNumbers()), result: \(board!.result())")
    } else {
        print("No boards with bingo")
    }
}

func parseLinesToNumbersAndBoard(lines: [String]) -> ([Int], [BingoBoard]) {
    let numbers = lines[0].split(separator: ",").map { Int($0)! }
    var boards: [BingoBoard] = []
    for index in 2 ... lines.count {
        if (index - 2) % 5 == 0, index > 3 {
            boards.append(from_lines(lines: lines[index - 6 ... index - 2].map { String($0) }))
        } else if index == lines.count {
            boards.append(from_lines(lines: lines[index - 5 ... index - 1].map { String($0) }))
        }
    }
    return (numbers, boards)
}

func from_lines(lines: [String]) -> BingoBoard {
    var newBoard: [[BingoCell]] = []
    for i in 0 ... lines.count - 1 {
        let numbers = lines[i].split(separator: " ").map { Int($0)! }
        newBoard.append(numbers.map { BingoCell(number: $0) })
    }
    return BingoBoard(board: newBoard)
}

func boardWithFirstBingo(numbers: [Int], boards: [BingoBoard]) -> BingoBoard? {
    for number in numbers {
        for board in boards {
            board.mark_number(number: number)
            if board.has_bingo() {
                return board
            }
        }
    }
    return nil
}

func boardWithLastBingo(numbers: [Int], boards: [BingoBoard]) -> BingoBoard? {
    var bingo_boards: [BingoBoard] = []
    for number in numbers {
        for board in boards {
            if board.has_bingo() {
                continue
            }
            board.mark_number(number: number)
            if board.has_bingo() {
                bingo_boards.append(board)
            }
        }
    }
    return bingo_boards.last
}
