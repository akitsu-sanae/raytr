#include <iostream>
#include <fstream>
#include "utility.hpp"
#include "mesh.hpp"

Mesh Mesh::load_file(const char* filename) {
    std::ifstream input{filename};
    std::string line;
    Mesh result;
    while (std::getline(input, line)) {
        auto data = util::split(line);
        if (data[0][0] == '#')
            continue;
        if (data[0] == "v") {
            auto pos = util::range<std::string>{data, 1, data.size()}
                .map([&](std::string const& s) {
                        return std::stod(s);
                })
                .to_array<3>();
            auto node = Node {
                result.nodes.size(),
                pos
            };
            result.nodes.push_back(std::move(node));
        }
        if (data[0] == "f") {
            auto node_ids = util::range<std::string>{data, 1, data.size()}
                .map([&](std::string const& s) {
                        return std::stoul(s) - 1;
                })
                .to_array<3>();

            auto face = Face {
                result.faces.size(),
                node_ids
            };
            result.faces.push_back(std::move(face));
        }
    }
    return result;
}

void Mesh::debug() const {
    std::cout << "nodes: " << std::endl;
    for (auto const& node : nodes) {
        std::cout <<
            util::format(
                    "    id: {}, x: {}, y: {}, z: {}",
                    node.id,
                    node.position[0],
                    node.position[1],
                    node.position[2])
            << std::endl;
    }

    std::cout << "faces: " << std::endl;
    for (auto const& face : faces) {
        std::cout <<
            util::format(
                    "    id: {}, x: {}, y: {}, z: {}",
                    face.id,
                    face.node_ids[0],
                    face.node_ids[1],
                    face.node_ids[2])
            << std::endl;
    }
}

