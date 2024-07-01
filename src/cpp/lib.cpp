#include <algorithm>
#include <charconv>
#include <execution>
#include <iterator>
#include <numeric>
#include <span>
#include <string>
#include <string_view>
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
    line_iterator(T begin, T end, char delim)
        : begin_(begin), end_(end), delim_(delim) {}

    class iterator {
        T begin_;
        T end_;
        char delim_;

      public:
        using iterator_category = std::forward_iterator_tag;
        using value_type = T *;
        using difference_type = size_t;
        using pointer_ = T *;
        using reference = T &;
        iterator(T begin, T end, char delim)
            : begin_(begin), end_(end), delim_(delim) {}

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

        bool operator!=(const iterator &other) {
            return begin_ != other.begin_;
        }
    };

    iterator begin() { return iterator(begin_, end_, delim_); }
    iterator end() { return iterator(end_, end_, delim_); }
};

size_t part1(std::string_view input) {
    auto it = line_iterator(input.begin(), input.end(), '\n');
    std::vector<std::string_view> vec;
    for (auto l : it) {
        vec.push_back(l);
    }
    size_t res = std::transform_reduce(std::execution::par_unseq, vec.cbegin(),
                                       vec.cend(), 0, std::plus{},
                                       [](std::string_view l) {
                                           auto [pattern, groups] = parse(l);
                                           return count(pattern, groups);
                                       });
    return res;
}

struct custom_key {
    std::string_view pattern;
    std::span<size_t> groups;
    bool operator==(const custom_key &other) const {
        return pattern == other.pattern &&
               std::equal(groups.begin(), groups.end(), other.groups.begin(),
                          other.groups.end());
    }
};

template <> struct std::hash<custom_key> {
    std::size_t operator()(const custom_key &k) const {
        using std::hash;
        using std::size_t;
        using std::string_view;

        return ((hash<string_view>()(k.pattern) ^
                 (hash<size_t>()(k.groups.size()) << 1)) >>
                1) ^
               (hash<size_t>()(k.groups[0]) << 1);
    }
};

inline custom_key make_key(const std::string_view &s,
                           const std::span<size_t> &t) {
    return {s, t};
}

size_t count_with_cache(std::unordered_map<custom_key, size_t> &cache,
                        const std::string_view &pattern,
                        const std::span<size_t> &groups) {
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

    custom_key key = make_key(pattern, groups);
    auto it = cache.find(key);
    if (it != cache.end()) {
        return it->second;
    }

    size_t res = 0;
    char c = pattern[0];

    if (c == '.' || c == '?') {
        res += count_with_cache(cache, pattern.substr(1), groups);
    }

    if ((c == '#' || c == '?') && (groups[0] <= pattern.size()) &&
        (pattern.substr(0, groups[0]).find('.') == std::string::npos) &&
        (groups[0] == pattern.size() || pattern[groups[0]] != '#')) {
        if (groups[0] == pattern.size()) {
            res += count_with_cache(
                cache, "", std::span(groups.begin() + 1, groups.end()));
        } else {

            res +=
                count_with_cache(cache, pattern.substr(groups[0] + 1),
                                 std::span(groups.begin() + 1, groups.end()));
        }
    }
    cache[key] = res;
    return res;
}

size_t count_part2(const std::string_view &pattern,
                   const std::span<size_t> &groups) {
    // join pattern five times into a string by ?
    std::string repeated_pattern;
    repeated_pattern.reserve(pattern.size() * 5);
    for (size_t i = 0; i < 5; i++) {
        repeated_pattern.append(pattern);
        repeated_pattern.push_back('?');
    }
    repeated_pattern.pop_back();
    // create a groups array with 5 repetitions of the original group
    std::vector<size_t> repeated_groups;
    repeated_groups.reserve(groups.size() * 5);
    for (size_t i = 0; i < 5; i++) {
        repeated_groups.insert(repeated_groups.end(), groups.begin(),
                               groups.end());
    }
    std::unordered_map<custom_key, size_t> cache;
    return count_with_cache(cache, repeated_pattern, repeated_groups);
}

size_t part2(std::string_view input) {
    auto it = line_iterator(input.begin(), input.end(), '\n');
    std::vector<std::string_view> vec;
    for (auto l : it) {
        vec.push_back(l);
    }
    size_t res = std::transform_reduce(std::execution::par_unseq, vec.cbegin(),
                                       vec.cend(), (size_t)0, std::plus{},
                                       [](std::string_view l) {
                                           auto [pattern, groups] = parse(l);
                                           return count_part2(pattern, groups);
                                       });
    return res;
}

extern "C" {
size_t part1_c(const char *input, size_t input_len) {
    return part1(std::string_view(input, input_len));
}
size_t part2_c(const char *input, size_t input_len) {
    return part2(std::string_view(input, input_len));
}
}