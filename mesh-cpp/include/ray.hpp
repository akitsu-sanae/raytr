#ifndef MESH_RAY_HPP
#define MESH_RAY_HPP

#include <memory>

#include "mesh.hpp"
#include "cache.hpp"
#include "color.hpp"

struct Ray {
    std::array<float, 3> origin;
    std::array<float, 3> direction;
    std::unique_ptr<Color> collosion(Mesh const& mesh, Cache const& cache) const {
        return std::make_unique<Color>(0, 0, 0);
    }
};

#endif

