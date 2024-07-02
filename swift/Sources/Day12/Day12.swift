// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct oneLine {
    let pattern: String
    let groups: [Int]
}

extension oneLine: Equatable {
    static func == (lhs: oneLine, rhs: oneLine) -> Bool {
        return lhs.pattern == rhs.pattern && lhs.groups == rhs.groups
    }
}

// parse a line of input into a tuple of (String, [Int])
// string and numbers are separated by a space, the numbers are separated by commas

func parse(line: String) -> oneLine {
    let parts = line.components(separatedBy: " ")
    let numbers = parts[1].components(separatedBy: ",").compactMap { Int($0) }
    return oneLine(pattern: parts[0], groups: numbers)
}

// parse the input given as Data into an array of (String, [Int]), one for each line
// lines are separated by newlines

func parse(data: Data) -> [oneLine] {
    let lines = String(data: data, encoding: .utf8)!.components(separatedBy: "\n")
    return lines.compactMap { line in
        if line.isEmpty { return nil }
        return parse(line: line)
    }
}

func count(pattern: any StringProtocol, groups: ArraySlice<Int>) -> Int {
    if pattern.isEmpty {
        if groups.isEmpty {
            return 1
        } else {
            return 0
        }
    }

    if groups.isEmpty {
        if pattern.firstIndex(of: "#") != nil {
            return 0
        } else {
            return 1
        }
    }

    var result = 0;

    if pattern.first == "." || pattern.first == "?" {
        result += count(pattern: pattern.dropFirst(), groups: groups)
    }

    let g0 = groups.first!
    if (pattern.first == "#" || pattern.first == "?") 
    && g0 <= pattern.count 
    && pattern.prefix(g0).firstIndex(of: ".") == nil
    && (pattern.count == g0 || pattern.prefix(g0 + 1).last! != "#")
        {
            result += count(pattern: pattern.dropFirst(g0 + 1), groups: groups.dropFirst())
        }
    

    return result;
}

func part1_single(data: Data) -> Int {
    let parsed = parse(data: data)
    return parsed.map { line in
        return count(pattern: line.pattern, groups: ArraySlice(line.groups))
    }.reduce(0, +)
}

func part1(data: Data) -> Int {
    let parsed = parse(data: data)
    var result = 0
    let syncQueue = DispatchQueue(label: "syncQueue")

    DispatchQueue.concurrentPerform(iterations: parsed.count) { i in
        let it = count(pattern: parsed[i].pattern, groups: ArraySlice(parsed[i].groups))
        syncQueue.sync {result += it}
    }

    return result
}

@_cdecl("part1_swift")
public func part1SwiftFFI(data: UnsafeMutablePointer<Int8>, length: Int) -> Int {
    let data = Data(bytesNoCopy: data, count: length, deallocator: .none)
    return part1(data: data)
}