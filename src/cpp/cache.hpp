#pragma once

#include <optional>
#include <span>
#include <string_view>

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

class CacheProtocol {
    public:
    virtual std::optional<size_t> get(const custom_key &key) = 0;
    virtual void set(const custom_key &key, size_t value) = 0;
};

class Cache : public CacheProtocol {
    std::unordered_map<custom_key, size_t> cache;

    std::optional<size_t> get(const custom_key &key) override {
        auto it = cache.find(key);
        if (it != cache.end()) {
            return it->second;
        }
        return std::nullopt;
    }

    void set(const custom_key &key, size_t value) override {
        cache[key] = value;
    }
};

class NoCache : public CacheProtocol {
    std::optional<size_t> get(const custom_key &key) override {
        return std::nullopt;
    }

    void set(const custom_key &key, size_t value) override {}
};   