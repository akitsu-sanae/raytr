#ifndef MESH_CACHE_HPP
#define MESH_CACHE_HPP

#include <memory>
#include <vector>

#include "boundary_box.hpp"
#include "face.hpp"

struct Mesh;

struct Cache {
    explicit Cache(BoundaryBox const& boundary_box) :
        boundary_box(boundary_box)
    {}
    virtual ~Cache() {}

    virtual void add(Face const&, Mesh const&) = 0;
    virtual std::string debug() const = 0;
    BoundaryBox boundary_box;
};

struct CacheNode : public Cache {
    explicit CacheNode(BoundaryBox const&);
    void add(Face const& face, Mesh const&) override;
    std::string debug() const override;
    std::array<std::unique_ptr<Cache>, 8> children;
};

struct CacheLeaf : public Cache {
    using Cache::Cache;
    void add(Face const& face, Mesh const&) override;
    std::string debug() const override;
    std::unique_ptr<Cache> divide(Mesh const&) const;

    std::vector<size_t> face_ids;
};

#endif

