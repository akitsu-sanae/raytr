#ifndef MESH_MESH_HPP
#define MESH_MESH_HPP

#include <vector>

#include "face.hpp"
#include "node.hpp"

struct BoundaryBox;

struct Mesh {
    static Mesh load_file(const char* filename);
    std::vector<Face> faces;
    std::vector<Node> nodes;

    void debug() const;
    BoundaryBox boundary_box() const;
};

#endif

