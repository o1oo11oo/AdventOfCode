use std::{collections::VecDeque, fmt::Display};

pub(crate) fn part_1(input: &str) -> String {
    let droplet = get_droplet(input);
    for (i, layer) in droplet.iter().enumerate() {
        log::debug!("layer {i}: \n{}", print_layer(layer).trim())
    }

    let mut sum = 0;
    for layer in &droplet {
        for row in layer {
            let mut last = Material::Air;
            for &voxel in row {
                if last != voxel {
                    sum += 1;
                }
                last = voxel;
            }
        }

        for z in 0..layer[0].len() {
            let mut last = Material::Air;
            for row in layer {
                let voxel = row[z];
                if last != voxel {
                    sum += 1;
                }
                last = voxel;
            }
        }
    }

    for z in 0..droplet[0][0].len() {
        for y in 0..droplet[0].len() {
            let mut last = Material::Air;
            for layer in &droplet {
                let voxel = layer[y][z];
                if last != voxel {
                    sum += 1;
                }
                last = voxel;
            }
        }
    }

    sum.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let mut droplet = get_droplet(input);
    flood_fill(&mut droplet);
    for (i, layer) in droplet.iter().enumerate() {
        log::debug!("layer {i}: \n{}", print_layer(layer).trim())
    }

    let mut sum = 0;
    for layer in &droplet {
        for row in layer {
            let mut last = Material::Steam;
            for &voxel in row {
                if last != voxel && last != Material::Air && voxel != Material::Air {
                    sum += 1;
                }
                last = voxel;
            }
        }

        for z in 0..layer[0].len() {
            let mut last = Material::Steam;
            for row in layer {
                let voxel = row[z];
                if last != voxel && last != Material::Air && voxel != Material::Air {
                    sum += 1;
                }
                last = voxel;
            }
        }
    }

    for z in 0..droplet[0][0].len() {
        for y in 0..droplet[0].len() {
            let mut last = Material::Steam;
            for layer in &droplet {
                let voxel = layer[y][z];
                if last != voxel && last != Material::Air && voxel != Material::Air {
                    sum += 1;
                }
                last = voxel;
            }
        }
    }

    sum.to_string()
}

fn get_droplet(input: &str) -> Vec<Vec<Vec<Material>>> {
    let coordinates = input.lines().map(Coordinate::from).collect::<Vec<_>>();
    let size = coordinates
        .iter()
        .map(|c| c.x)
        .max()
        .unwrap()
        .max(coordinates.iter().map(|c| c.y).max().unwrap())
        .max(coordinates.iter().map(|c| c.z).max().unwrap())
        + 2;

    let mut droplet = vec![vec![vec![Material::Air; size]; size]; size];
    for c in &coordinates {
        droplet[c.x][c.y][c.z] = Material::Lava;
    }

    droplet
}

fn flood_fill(droplet: &mut [Vec<Vec<Material>>]) {
    let mut queue = VecDeque::from([Coordinate { x: 0, y: 0, z: 0 }]);
    droplet[0][0][0] = Material::Steam;
    let size = droplet.len();

    while let Some(c) = queue.pop_front() {
        for n in c.neighbours(size - 1) {
            let c = &mut droplet[n.x][n.y][n.z];
            if *c == Material::Air {
                *c = Material::Steam;
                queue.push_back(n);
            }
        }
    }
}

fn print_layer(layer: &[Vec<Material>]) -> String {
    layer
        .iter()
        .map(|row| row.iter().map(|&voxel| voxel.to_string()))
        .fold(String::new(), |mut acc, c| {
            acc.extend(c);
            acc += "\n";
            acc
        })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Material {
    Air,
    Lava,
    Steam,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Air => write!(f, " "),
            Material::Lava => write!(f, "#"),
            Material::Steam => write!(f, "."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn neighbours(&self, clamp: usize) -> impl Iterator<Item = Coordinate> + '_ {
        [1, usize::MAX, 0, 0, 0, 0]
            .iter()
            .zip([0, 0, 1, usize::MAX, 0, 0].iter())
            .zip([0, 0, 0, 0, 1, usize::MAX].iter())
            .map(move |((&x, &y), &z)| Coordinate {
                x: self.x.wrapping_add(x).clamp(0, clamp),
                y: self.y.wrapping_add(y).clamp(0, clamp),
                z: self.z.wrapping_add(z).clamp(0, clamp),
            })
            .filter(move |c| c != self)
    }
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        let mut nums = value.split(',').map(|n| n.parse().unwrap());
        Coordinate {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
        }
    }
}
