
#include <cstdio>
#include <fstream>
#include <iostream>
#include <istream>
#include <span>
#include <sstream>
#include <string>
#include <string_view>
#include <sys/_types/_size_t.h>
#include <tuple>
#include <vector>
#include <strstream>

std::tuple<std::string, std::vector<size_t>> parse(std::string line) {
    std::istringstream iss(line);
    std::string pattern;
    getline(iss, pattern, ' ');
    std::string groups;
    getline(iss, groups, ' ');
    std::vector<size_t> vec;
    std::istringstream iss2(groups);
    std::string num;
    while (getline(iss2, num, ','))
        vec.push_back(std::stoi(num));
    return std::make_tuple(pattern, vec);
}

size_t count(const std::string &pattern, const std::span<size_t> &groups) {
    if (pattern.empty()) {
        if (groups.empty()) {
            return 1;
        } else {
            return 0;
        }
    }
    if (groups.empty()) {
        if (pattern.find('#') != std::string::npos) {
            return 0;
        } else {
            return 1;
        }
    }

    size_t res = 0;
    char c = pattern[0];

    if (c == '.' || c == '?') {
        res += count(pattern.substr(1), groups);
    }

    if ((c == '#' || c == '?') && (groups[0] <= pattern.size()) &&
        (pattern.substr(0, groups[0]).find('.') == std::string::npos) &&
        (groups[0] == pattern.size() || pattern[groups[0]] != '#')) {
        if (groups[0] == pattern.size()) {
            res += count("", std::span(groups.begin() + 1, groups.end()));
        } else {

            res += count(pattern.substr(groups[0] + 1),
                         std::span(groups.begin() + 1, groups.end()));
        }
    }
    return res;
}

size_t part1(std::span<char> input) {
    std::string line;
    size_t res = 0;
    std::istrstream in(input.data(), input.size());
    while (std::getline(in, line)) {
        auto [pattern, groups] = parse(line);
        res += count(pattern, groups);
    };
    return res;
}

size_t part2(std::span<char> input) { return 0; }

extern "C" {
size_t part1_c(char *input, size_t input_len) {
    return part1(std::span<char>(input, input_len));
}
size_t part2_c(char *input, size_t input_len) {
    return part2(std::span<char>(input, input_len));
}
}

