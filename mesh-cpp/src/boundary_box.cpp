#include "boundary_box.hpp"
#include "matrix.hpp"
#include "mesh.hpp"

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



