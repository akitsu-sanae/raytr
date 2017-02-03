#include <iostream>
#include <utility>
#include <png++/png.hpp>

#include "mesh.hpp"
#include "ray.hpp"

int main() {
    auto mesh = Mesh::load_file("bunny.obj");
    auto cache = Cache{mesh};
    png::image<png::rgb_pixel> image{800, 800};
    auto scale = std::make_pair(4.0 / 800.0, 4.0 / 800.0);
    vector ray_origin {{
        0.0, 0.0, -0.1
    }};
    for (size_t y = 0; y < image.get_height(); ++y) {
        for (size_t x = 0; x < image.get_width(); ++x) {
            auto const ray = Ray {
                ray_origin,
                vector {{
                    (double)x * scale.first - 2.0,
                    (double)y * scale.second - 2.0,
                    1.0
                }}
            };
            auto color = ray.collision(mesh, cache);
            if (color) {
                image[y][x] = png::rgb_pixel{
                    color->r, color->g, color->b
                };
            }
        }
    }
    image.write("output.png");
}

