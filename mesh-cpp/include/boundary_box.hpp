#ifndef MESH_BOUNDARY_BOX_HPP
#define MESH_BOUNDARY_BOX_HPP

struct Face;
struct Mesh;
struct Ray;

struct BoundaryBox {
    struct Range {
        double max;
        double min;

        double ave() const {
            return (max + min) / 2.0;
        }
    };
    Range x;
    Range y;
    Range z;

    bool collision(Face const&, Mesh const&) const;
};


#endif

