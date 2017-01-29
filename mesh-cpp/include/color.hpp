#ifndef MESH_COLOR_HPP
#define MESH_COLOR_HPP

struct Color {
    unsigned char r, g, b;
    explicit Color(
            unsigned char r,
            unsigned char g,
            unsigned char b) :
        r{r}, g{g}, b{b}
    {}
};

#endif

