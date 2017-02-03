#include "cache.hpp"
#include "mesh.hpp"

CacheNode::CacheNode(BoundaryBox const& boundary_box) :
    Cache(boundary_box)
{
    for (size_t i=0; i<8; i++) {
        auto tmp = i;
        auto child_box = boundary_box;
        if (tmp%2)
            child_box.x.min = boundary_box.x.ave();
        else
            child_box.x.max = boundary_box.x.ave();
        tmp /= 2;
        if (tmp%2)
            child_box.y.min = boundary_box.y.ave();
        else
            child_box.y.max = boundary_box.y.ave();
        tmp /= 2;
        if (tmp%2)
            child_box.z.min = boundary_box.z.ave();
        else
            child_box.z.max = boundary_box.z.ave();
        children[i] = std::make_unique<CacheLeaf>(child_box);
    }
}

void CacheNode::add(Face const& face, Mesh const& mesh) {
    if (!boundary_box.collision(face, mesh))
        return;

    for (auto& child : children) {
        child->add(face, mesh);
        auto leaf = dynamic_cast<CacheLeaf const*>(child.get());
        if (leaf) {
            if (leaf->face_ids.size() > 64)
                child = leaf->divide(mesh);
        }
    }
}

std::string CacheNode::debug() const {
    std::string result;
    result += "node {\n";
    for (auto const& child : children) {
        result += child->debug() + "\n";
    }
    result += "}";
    return result;
}

void CacheLeaf::add(Face const& face, Mesh const& mesh) {
    if (!boundary_box.collision(face, mesh))
        return;
    face_ids.push_back(face.id);
}

std::unique_ptr<Cache> CacheLeaf::divide(Mesh const& mesh) const {
    auto result = std::make_unique<CacheNode>(boundary_box);
    for (auto const& face_id : face_ids) {
        auto const& face = mesh.faces[face_id];
        result->add(face, mesh);
    }
    return result;
}

std::string CacheLeaf::debug() const {
    std::string result = "leaf: ";
    for (auto const& id : face_ids) {
        result += std::to_string(id) + ", ";
    }
    return result;
}

