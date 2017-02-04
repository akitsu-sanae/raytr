#include "ray.hpp"

#include <iostream>


std::unique_ptr<Color>
Ray::collision(
        Mesh const& mesh,
        std::unique_ptr<Cache> const& cache) const
{
    auto face_ids = cache->get(*this);
    for (auto const& face_id : face_ids) {
        auto const& face = mesh.faces[face_id];
        std::array<vector, 3> node_poss;
        for (size_t i=0; i<3; i++) {
            auto node_id = face.node_ids[i];
            auto const& node = mesh.nodes[node_id];
            node_poss[i] = node.position;
        }
        if (collision_impl(node_poss))
            return std::make_unique<Color>(200, 100, 0);
    }
    return nullptr;
}


bool Ray::collision_impl(std::array<vector, 3> const& nodes) const {
    auto a = nodes[1] - nodes[0];
    auto b = nodes[2] - nodes[0];
    auto r = -direction;

    auto denominator = Matrix{{
        a, b, r
    }}.determinant();

    if (denominator <= 0.0)
        return false;

    auto d = origin - nodes[0];

    auto u = Matrix{{
        d, b, r
    }}.determinant() / denominator;

    auto v = Matrix{{
        a, d, r
    }}.determinant() / denominator;

    auto t = Matrix{{
        a, b, d
    }}.determinant();

    if (u < 0.0 || u > 1.0)
        return false;
    if (v < 0.0 || v > 1.0)
        return false;
    if (u+v < 0.0 || u+v > 1.0)
        return false;
    if (t < 0.0)
        return false;
    return true;
}
