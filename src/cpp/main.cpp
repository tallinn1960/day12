#include <vector>
#include <fstream>
#include <iostream>
#include <span>

extern size_t part1(std::span<char> input);
extern size_t part2(std::span<char> input);

static std::vector<char> ReadAllBytes(char const *filename) {
    std::ifstream ifs(filename, std::ios::binary | std::ios::ate);
    std::ifstream::pos_type pos = ifs.tellg();

    if (pos == 0) {
        return std::vector<char>{};
    }

    std::vector<char> result(pos);

    ifs.seekg(0, std::ios::beg);
    ifs.read(&result[0], pos);

    return result;
}

int main() {
    std::vector<char> input = ReadAllBytes("../../input.txt");
    std::cout << part1(input) << std::endl;
    std::cout << part2(input) << std::endl;
    return 0;
}
