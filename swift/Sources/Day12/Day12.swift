// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct OneLine {
    let pattern: Substring
    let groups: [Int]
}

extension OneLine: Hashable {
    func hash(into hasher: inout Hasher) {
        hasher.combine(pattern)
        hasher.combine(groups)
    }
}

extension OneLine: Equatable {
    static func == (lhs: OneLine, rhs: OneLine) -> Bool {
        return lhs.pattern == rhs.pattern && lhs.groups == rhs.groups
    }
}

// parse a line of input into a tuple of (String, [Int])
// string and numbers are separated by a space, the numbers are separated by commas

func parse(line: Substring) -> OneLine {
    let parts = line.components(separatedBy: " ")
    let numbers = parts[1].components(separatedBy: ",").compactMap { Int($0) }
    return OneLine(pattern: parts[0][...], groups: numbers)
}

func count(_ cache: inout [OneLine: Int], pattern: Substring, groups: ArraySlice<Int>) -> Int {
    if pattern.isEmpty {
        if groups.isEmpty {
            return 1
        } else {
            return 0
        }
    }

    if groups.isEmpty {
        if pattern.contains("#") {
            return 0
        } else {
            return 1
        }
    }

    let key = OneLine(pattern: pattern, groups: Array(groups))
    if let cached = cache[key] {
        return cached
    }

    var result = 0;

    if pattern.first == "." || pattern.first == "?" {
        result += count(&cache, pattern: pattern.dropFirst(), groups: groups)
    }

    let g0 = groups.first!
    if (pattern.first == "#" || pattern.first == "?")
        && g0 <= pattern.count
        && !pattern.prefix(g0).contains(".")
        && (pattern.count == g0 || pattern.prefix(g0 + 1).last! != "#")
    {
        result += count(&cache, pattern: pattern.dropFirst(g0 + 1), groups: groups.dropFirst())
    }

    cache[key] = result
    return result;
}

public func part1(data: Data) -> Int {
    let lines = Substring(decoding: data, as: UTF8.self).split(separator: "\n")
    var result = 0
    let syncQueue = DispatchQueue(label: "syncQueue")

    DispatchQueue.concurrentPerform(iterations: lines.count) { i in
        if lines[i].isEmpty { return }
        let parsed = parse(line: lines[i])
        var cache: [OneLine: Int] = [:]
        let it = count(&cache, pattern: parsed.pattern[...], groups: ArraySlice(parsed.groups))
        syncQueue.sync { result += it }
    }

    return result
}

public func part2(data: Data) -> Int {
    let lines = Substring(decoding: data, as: UTF8.self).split(separator: "\n")
    var result = 0
    let syncQueue = DispatchQueue(label: "syncQueue")

    DispatchQueue.concurrentPerform(iterations: lines.count) { i in
        if lines[i].isEmpty { return }
        let parsed = parse(line: lines[i])
        let pattern = (0..<5).map { _ in parsed.pattern }.joined(separator: "?")
        let groups = (0..<5).flatMap { _ in parsed.groups }
        var cache: [OneLine: Int] = [:]

        let it = count(&cache, pattern: pattern[...], groups: ArraySlice(groups))
        syncQueue.sync { result += it }
    }

    return result
}

@_cdecl("part1_swift")
public func part1SwiftFFI(data: UnsafeMutablePointer<Int8>, length: Int) -> Int {
    let data = Data(bytesNoCopy: data, count: length, deallocator: .none)
    return part1(data: data)
}

@_cdecl("part2_swift")
public func part2SwiftFFI(data: UnsafeMutablePointer<Int8>, length: Int) -> Int {
    let data = Data(bytesNoCopy: data, count: length, deallocator: .none)
    return part2(data: data)
}
