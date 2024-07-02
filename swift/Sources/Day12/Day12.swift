// The Swift Programming Language
// https://docs.swift.org/swift-book
import Foundation

struct OneLine {
    let pattern: Substring
    let groups: ArraySlice<Int>
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
    return OneLine(pattern: parts[0][...], groups: numbers[...])
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

    let key = OneLine(pattern: pattern, groups: groups)
    if let cached = cache[key] {
        return cached
    }

    var result = 0

    if pattern.first == "." || pattern.first == "?" {
        result += count(&cache, pattern: pattern.dropFirst(), groups: groups)
    }

    if pattern.first == "#" || pattern.first == "?" {
        let nextGroup = groups.first!
        let match = pattern.prefix(nextGroup)
        let trail = pattern.dropFirst(nextGroup)
        if nextGroup == match.count
            && !match.contains(".")
            && (trail.isEmpty || trail.first! != "#")
        {
            // match is a valid group of g0 defective springs
            // handle the trail with the remaining groups
            // skip the first char of trail as that ends
            // the current group and is already matched
            result += count(&cache, pattern: trail.dropFirst(), groups: groups.dropFirst())
        }
    }

    cache[key] = result
    return result
}

public func part1(data: Data) -> Int {
    let lines = Substring(decoding: data, as: UTF8.self).split(separator: "\n")
    var result = 0
    let syncQueue = DispatchQueue(label: "syncQueue")

    DispatchQueue.concurrentPerform(iterations: lines.count) { index in
        if lines[index].isEmpty { return }
        let parsed = parse(line: lines[index])
        var cache: [OneLine: Int] = [:]
        let patternMatches = count(
            &cache, pattern: parsed.pattern[...], groups: ArraySlice(parsed.groups))
        syncQueue.sync { result += patternMatches }
    }

    return result
}

public func part2(data: Data) -> Int {
    let lines = Substring(decoding: data, as: UTF8.self).split(separator: "\n")
    var result = 0
    let syncQueue = DispatchQueue(label: "syncQueue")

    DispatchQueue.concurrentPerform(iterations: lines.count) { index in
        if lines[index].isEmpty { return }
        let parsed = parse(line: lines[index])
        let pattern = (0..<5).map { _ in parsed.pattern }.joined(separator: "?")
        let groups = (0..<5).flatMap { _ in parsed.groups }
        var cache: [OneLine: Int] = [:]

        let patternMatches = count(&cache, pattern: pattern[...], groups: ArraySlice(groups))
        syncQueue.sync { result += patternMatches }
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
