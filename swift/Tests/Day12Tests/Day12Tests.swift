import XCTest
@testable import Day12

final class Day12Tests: XCTestCase {
    func test_OneLine() {
        let line = OneLine(pattern: "???.###", groups: [1, 1, 3])
        var cache: [OneLine: Int] = [:]

        let result = count(&cache, pattern: line.pattern[...], groups: line.groups[...])
        XCTAssertEqual(1, result)
    }

    func test_sample() throws {
        let data = """
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            """.data(using: .utf8)!

        let parsed = Substring(decoding: data, as: UTF8.self).split(separator: "\n").map(parse)

        XCTAssertEqual(
            parsed,
            [
                OneLine(pattern: "???.###", groups: [1, 1, 3]),
                OneLine(pattern: ".??..??...?##.", groups: [1, 1, 3]),
                OneLine(pattern: "?#?#?#?#?#?#?#?", groups: [1, 3, 1, 6]),
                OneLine(pattern: "????.#...#...", groups: [4, 1, 1]),
                OneLine(pattern: "????.######..#####.", groups: [1, 6, 5]),
                OneLine(pattern: "?###????????", groups: [3, 2, 1]),
            ])

        var cache: [OneLine: Int] = [:]

        let result = parsed.map { line in
            return count(&cache, pattern: line.pattern[...], groups: ArraySlice(line.groups))
        }.reduce(0, +)
        XCTAssertEqual(result, 21)
    }

    func test_part2() {
        let data = """
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            """.data(using: .utf8)!
        let result = part2(data: data)
        XCTAssertEqual(result, 525152)
    }

}

final class Day12Performance: XCTestCase {
    func test_part1() {
        let data = try! Data(contentsOf: URL(fileURLWithPath: "../input.txt"))
        #if os(macOS)
            measure(metrics: [XCTClockMetric(), XCTCPUMetric(), XCTMemoryMetric()]) {
                let result = part1(data: data)
                XCTAssertEqual(result, 7221)
            }
        #else
            measure {
                let result = part1(data: data)
                XCTAssertEqual(result, 7221)
            }
        #endif
    }

    func test_part2() {
        let data = try! Data(contentsOf: URL(fileURLWithPath: "../input.txt"))
        #if os(macOS)
            measure(metrics: [XCTClockMetric(), XCTCPUMetric(), XCTMemoryMetric()]) {
                let result = part2(data: data)
                XCTAssertEqual(result, 7139671893722)
            }
        #else
            measure {
                let result = part2(data: data)
                XCTAssertEqual(result, 7139671893722)
            }
        #endif
    }
}
