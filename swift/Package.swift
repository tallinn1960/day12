// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Day12",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "Day12",
            type: .dynamic,
            targets: ["Day12"]),
        .executable(name: "Day12Exe", targets: ["Day12Exe"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "Day12"),
        .executableTarget(name: "Day12Exe", dependencies: ["Day12"]),
        .testTarget(
            name: "Day12Tests",
            dependencies: ["Day12"]),
    ]
)
