#ifndef MESH_RAY_HPP
#define MESH_RAY_HPP

#include <memory>

#include "mesh.hpp"
#include "cache.hpp"
#include "color.hpp"
#include "matrix.hpp"

struct Ray {
    vector origin;
    vector direction;

    std::unique_ptr<Color> collision(Mesh const& mesh, std::unique_ptr<Cache> const&) const;

    bool collision_impl(std::array<vector, 3> const&) const;
};

#endif

