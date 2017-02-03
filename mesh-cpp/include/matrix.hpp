#ifndef MESH_MATRIX_HPP
#define MESH_MATRIX_HPP

#include <array>

using vector = std::array<double, 3>;

inline static vector operator-(vector const& lhs, vector const& rhs) {
    return vector{{
        lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]
    }};
}

inline static vector operator-(vector const& v) {
    return vector{{
        -v[0], -v[1], -v[2]
    }};
}

struct Matrix {
    // data[0][0], data[1][0], data[2][0]
    // data[0][1], data[1][1], data[2][1]
    // data[0][2], data[1][2], data[2][2]
    std::array<std::array<double, 3>, 3> data;

    double determinant() const {
        double result = 0.0;
        result += data[0][0]*(data[1][1]*data[2][2] - data[2][1]*data[1][2]);
        result -= data[1][0]*(data[0][1]*data[2][2] - data[2][1]*data[0][2]);
        result += data[2][0]*(data[0][1]*data[1][2] - data[1][1]*data[0][2]);
        return result;
    }

};


#endif

