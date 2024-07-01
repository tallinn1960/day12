#include <fstream>
#include <iostream>
#include <string>
#include <string_view>

size_t part1(std::string_view input);

int main() {
    std::ifstream in("input.txt");
    std::string input;
    // read file into string
    in.seekg(0, std::ios::end);
    input.reserve(in.tellg());
    in.seekg(0, std::ios::beg);
    input.assign((std::istreambuf_iterator<char>(in)),
                 std::istreambuf_iterator<char>());


    in.close();
    size_t result = part1(input);
    std::cout << result << std::endl;
    return 0;
}