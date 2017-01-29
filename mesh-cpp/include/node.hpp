#ifndef MESH_NODE_HPP
#define MESH_NODE_HPP

#include <array>

struct Node {
    size_t id;
    std::array<float, 3> position;
};

#endif

