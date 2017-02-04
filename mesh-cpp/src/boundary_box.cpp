#include "boundary_box.hpp"
#include "matrix.hpp"
#include "mesh.hpp"
#include "ray.hpp"

bool BoundaryBox::collision(Face const& face, Mesh const& mesh) const {
    auto is_inner = [&](vector const& pos) {
        if (pos[0] <= x.min || x.max <= pos[0])
            return false;
        if (pos[1] <= y.min || y.max <= pos[1])
            return false;
        if (pos[2] <= z.min || z.max <= pos[2])
            return false;
        return true;
    };
    for (auto const& node_id : face.node_ids) {
        auto const& node = mesh.nodes[node_id];
        if (is_inner(node.position))
            return true;
    }
    return false;
}

bool BoundaryBox::collision(Ray const& ray) const {
    // projection to XY plane
    // y - ray.origin[1] = (ray.dir[1]/ray.dir[0])(x - ray.origin[0])
    auto left_y = (ray.direction[1]/ray.direction[0])*(x.min - ray.origin[0]) + ray.origin[1];
    auto right_y = (ray.direction[1]/ray.direction[0])*(x.max - ray.origin[0]) + ray.origin[1];
    if (left_y < y.min && right_y < y.min)
        return false;
    if (y.max < left_y && y.max < right_y)
        return false;

    // projection to XZ plane
    // z - ray.origin[2] = (ray.dir[2]/ray.dir[0])(x - ray.origin[0])
    auto left_z = (ray.direction[2]/ray.direction[0])*(x.min - ray.origin[0]) + ray.origin[2];
    auto right_z = (ray.direction[2]/ray.direction[0])*(x.max - ray.origin[0]) + ray.origin[2];
    if (left_z < z.min && right_z < z.min)
        return false;
    if (z.max < left_z && z.max < right_z)
        return false;
    return true;
}

