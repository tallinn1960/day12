import XCTest
@testable import Day12

final class Day12Tests: XCTestCase {
    func test_oneLine() {
        let line = oneLine(pattern: "???.###", groups: [1, 1, 3])
        let result = count(pattern: line.pattern, groups: ArraySlice(line.groups)) 
        XCTAssertEqual(result, 1)
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

        let parsed = parse(data: data)
        XCTAssertEqual(parsed, [
            oneLine(pattern: "???.###", groups: [1, 1, 3]),
            oneLine(pattern: ".??..??...?##.", groups: [1, 1, 3]),
            oneLine(pattern: "?#?#?#?#?#?#?#?", groups: [1, 3, 1, 6]),
            oneLine(pattern: "????.#...#...", groups: [4, 1, 1]),
            oneLine(pattern: "????.######..#####.", groups: [1, 6, 5]),
            oneLine(pattern: "?###????????", groups: [3, 2, 1]),
        ])

        
        let result = parsed.map {  line in
            return count(pattern: line.pattern, groups: ArraySlice(line.groups))
        }.reduce(0, +)
        XCTAssertEqual(result, 21)
    }
}
