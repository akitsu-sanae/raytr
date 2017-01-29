#ifndef MESH_FACE_HPP
#define MESH_FACE_HPP

#include <array>

struct Face {
    size_t id;
    std::array<size_t, 3> node_ids;
};

#endif

