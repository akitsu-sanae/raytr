#ifndef MESH_UTILITY_HPP
#define MESH_UTILITY_HPP

#include <string>
#include <sstream>
#include <array>
#include <vector>

namespace util {

std::vector<std::string> split(std::string const& str) {
    std::vector<std::string> res;
    std::stringstream ss{str};
    std::string buf;
    while (std::getline(ss, buf, ' '))
        res.push_back(buf);
    return res;
}

struct format_too_many_args {};
std::string format(std::string const& str) {
    return str;
}

template<typename Arg, typename ... Args>
std::string format(std::string const& str, Arg&& arg, Args&& ... args) {
    auto pos = str.find("{}");
    if (pos == std::string::npos)
        throw format_too_many_args{};
    std::string before = str.substr(0, pos);
    std::string after = str.substr(pos + 2, str.length());
    std::stringstream ss;
    ss << std::forward<Arg>(arg);
    return before + ss.str() + format(after, args ...);
}

template<typename T>
struct range {
    std::vector<T> data;
    size_t first;
    size_t last;

    template<typename F>
    auto map(F const& f) const {
        using result_t = decltype(f(data[first]));
        std::vector<result_t> result;
        for (size_t i=first; i < last; i++)
            result.push_back(f(data[i]));
        return range<result_t>{result, 0, result.size()};
    }

    template<size_t N>
    std::array<T, N> to_array() const {
        std::array<T, N> result;
        for (size_t i=0; i<N; i++)
            result[i] = data[i];
        return result;
    }

};

}



#endif

