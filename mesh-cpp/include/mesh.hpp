#ifndef MESH_MESH_HPP
#define MESH_MESH_HPP

#include <vector>

#include "face.hpp"
#include "node.hpp"

struct Mesh {
    static Mesh load_file(const char* filename);
    std::vector<Face> faces;
    std::vector<Node> nodes;

    void debug() const;
};

#endif

