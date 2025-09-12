pub enum GeometricalShapes {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

pub enum GeometricalVolumes {
    Cube,
    Sphere,
    Cone,
    TriangularPyramid,
    Parallelepiped,
}

pub(crate) fn square_area(side: usize) -> usize {
    side.pow(2)
}

pub(crate) fn triangle_area(base: usize, height: usize) -> f64 {
    (base as f64 * height as f64) / 2.0
}

pub(crate) fn circle_area(radius: usize) -> f64 {
    std::f64::consts::PI * (radius.pow(2) as f64)
}

pub(crate) fn rectangle_area(side_a: usize, side_b: usize) -> usize {
    side_a * side_b
}

pub(crate) fn cube_volume(side: usize) -> usize {
    side.pow(3)
}

pub(crate) fn sphere_volume(radius: usize) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * (radius.pow(3) as f64)
}

pub(crate) fn triangular_pyramid_volume(base_area: f64, height: usize) -> f64 {
    (base_area * height as f64) / 3.0
}

pub(crate) fn parallelepiped_volume(side_a: usize, side_b: usize, side_c: usize) -> usize {
    side_a * side_b * side_c
}

pub(crate) fn cone_volume(base_radius: usize, height: usize) -> f64 {
    (1.0 / 3.0) * std::f64::consts::PI * base_radius.pow(2) as f64 * height as f64
}

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area_r = (x * y) as f64;
    match kind {
        GeometricalShapes::Square => {
            let sq_area = square_area(a) as f64;
            if sq_area * times as f64 <= area_r {
                return true;
            } else {
                return false;
            }
        }
        GeometricalShapes::Circle => {
            let c_area = circle_area(a) as f64;
            if c_area * times as f64 <= area_r {
                return true;
            } else {
                return false;
            }
        }
        GeometricalShapes::Rectangle => {
            let rec_area = rectangle_area(a, b) as f64;
            if rec_area * times as f64 <= area_r {
                return true;
            } else {
                return false;
            }
        }
        GeometricalShapes::Triangle => {
            let rec_area = triangle_area(a, b) as f64;
            if rec_area * times as f64 <= area_r {
                return true;
            } else {
                return false;
            }
        }
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let v_box = (x * y * z) as f64;
    match kind {
        GeometricalVolumes::Cube => {
            let v_cube = cube_volume(a) as f64;
            if v_cube * times as f64 <= v_box {
                return true;
            } else {
                return false;
            }
        }
        GeometricalVolumes::Sphere => {
            let v_sph = sphere_volume(a) as f64;
            if v_sph * times as f64 <= v_box {
                return true;
            } else {
                return false;
            }
        }
        GeometricalVolumes::Cone => {
            let v_cone = cone_volume(a,b) as f64;
            if v_cone * times as f64 <= v_box {
                return true;
            } else {
                return false;
            }
        }
        GeometricalVolumes::TriangularPyramid => {
            let v_tp = triangular_pyramid_volume(a as f64, b) as f64;
            if v_tp * times as f64 <= v_box {
                return true;
            } else {
                return false;
            }
        }
        GeometricalVolumes::Parallelepiped => {
            let v_pll = parallelepiped_volume(a, b, c) as f64;
            if v_pll * times as f64 <= v_box {
                return true;
            } else {
                return false;
            }
        }
    }
}
