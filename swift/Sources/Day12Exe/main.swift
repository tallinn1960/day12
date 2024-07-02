//
//  File.swift
//  
//
//  Created by Olaf Schlüter on 02.07.24.
//

import Foundation
import Day12

let data = try! Data(contentsOf: URL(fileURLWithPath: "../../../../input.txt"))

var result = part1(data: data)
print("\(result)")
result = part2(data: data)
print("\(result)")

