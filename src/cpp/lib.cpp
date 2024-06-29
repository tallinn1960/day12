
#include <charconv>
#include <cstddef>
#include <span>
#include <string>
#include <string_view>
#include <strstream>
#include <tuple>
#include <vector>

std::tuple<std::string_view, std::vector<size_t>> parse(std::string_view line) {

    std::string_view pattern;
    std::string_view groups;

    auto p = line.find(' ');
    pattern = line.substr(0, p);
    groups = line.substr(p + 1);

    std::vector<size_t> vec;
    size_t pos = 0;
    while (pos < groups.size()) {
        auto p = groups.find(',', pos);
        if (p == std::string::npos) {
            p = groups.size();
        }
        size_t val;
        std::from_chars(groups.data() + pos, groups.data() + p, val);
        vec.push_back(val);
        pos = p + 1;
    }

    return std::make_tuple(pattern, vec);
}

size_t count(const std::string_view &pattern, const std::span<size_t> &groups) {
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


template <typename T> class line_iterator {
    T begin_;
    T end_;
    char delim_;

  public:
    line_iterator(T begin, T end, char delim) : begin_(begin), end_(end), delim_(delim) {}

    class iterator {
        T begin_;
        T end_;
        char delim_;

      public:
        iterator(T begin, T end, char delim) : begin_(begin), end_(end), delim_(delim) {}

        iterator &operator++() {
            begin_ = std::find(begin_, end_, delim_);
            if (begin_ != end_) {
                ++begin_;
            }
            return *this;
        }

        std::string_view operator*() {
            auto end = std::find(begin_, end_, delim_);
            return std::string_view(&*begin_, end - begin_);
        }

        bool operator!=(const iterator &other) { return begin_ != other.begin_; }
    };

    iterator begin() { return iterator(begin_, end_, delim_); }
    iterator end() { return iterator(end_, end_, delim_); }
};

size_t part1(std::span<const char> input) {
    std::string line;
    size_t res = 0;
    for (auto l : line_iterator(input.begin(), input.end(), '\n')) {
        auto [pattern, groups] = parse(l);
        res += count(pattern, groups);
    }
    return res;
}

size_t part2(std::span<const char> input) { return 0; }

extern "C" {
size_t part1_c(const char *input, size_t input_len) {
    return part1(std::span<const char>(input, input_len));
}
size_t part2_c(const char *input, size_t input_len) {
    return part2(std::span<const char>(input, input_len));
}
} 