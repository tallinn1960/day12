//
//  File.swift
//
//
//  Created by Olaf Schlüter on 02.07.24.
//

import Foundation
import Day12

let arguments = CommandLine.arguments.count
if arguments < 2 {
    print("Usage: \(CommandLine.arguments[0]) <filename>")
    exit(1)
}

let filename = CommandLine.arguments[1]

let data = try! Data(contentsOf: URL(fileURLWithPath: filename))

var result = part1(data: data)
print("\(result)")
result = part2(data: data)
print("\(result)")
