use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};

use std::collections::HashMap;

use stpsyr::types::*;

enum LookState {
    Nothing,
    Provinces,
    Units
}

impl Stpsyr {

    pub fn render_svg(&self, path: String) -> io::Result<()> {
        let in_file = BufReader::new(File::open("data/standard.svg")?);
        let mut out_file = File::create(path)?;
        let mut look_state = LookState::Nothing;
        let mut style_line = String::new();
        let mut data_line = String::new();
        let mut province_centers = HashMap::new();

        for line in in_file.lines() {
            let mut line = line?;
            match look_state {

                _ if line.trim() == "</g>" => look_state = LookState::Nothing,

                LookState::Provinces if line.trim().starts_with("id=\"") => {
                    // apply the correct color
                    let quote_idx = line.find('"').unwrap() + 1;
                    let province = Province::from(line.chars().skip(quote_idx)
                            .take(3).collect::<String>());

                    let color = if let Some(p) = self.map.iter()
                            .find(|&r| r.province == province) {
                        if p.army_borders.is_empty() { "c5dfea" }
                        else if p.owner == Some(Power::from("Russia"))  { "a87e9f" }
                        else if p.owner == Some(Power::from("Austria")) { "c48f85" }
                        else if p.owner == Some(Power::from("Turkey"))  { "eaeaaf" }
                        else if p.owner == Some(Power::from("Italy"))   { "a4c499" }
                        else if p.owner == Some(Power::from("France"))  { "79afc6" }
                        else if p.owner == Some(Power::from("Germany")) { "a08a75" }
                        else if p.owner == Some(Power::from("England")) { "efc4e4" }
                        else { "e2c69e" }
                    } else { "000000" };

                    let hash_idx = style_line.find('#').unwrap() + 1;
                    write!(out_file, "{}{}",
                           style_line.drain(..hash_idx).collect::<String>(),
                           color)?;

                    style_line.drain(..6);
                    writeln!(out_file, "{}", style_line)?;

                    // find the vertices of the polygon
                    let mut points = vec![];
                    let mut mode = 'L';
                    let (mut last_x, mut last_y) = (0f32, 0f32);
                    for rule in data_line.split_whitespace().skip(1) {
                        if rule.len() <= 2 {
                            mode = rule.chars().next().unwrap();
                        } else {
                            match mode {
                                'H' => last_x = rule.parse().unwrap(),
                                'V' => last_y = rule.parse().unwrap(),
                                'L' => {
                                    let mut rule = rule.split(',');
                                    last_x = rule.next().unwrap().parse().unwrap();
                                    last_y = rule.next().unwrap().parse().unwrap();
                                },
                                _ => unreachable!()
                            }
                            points.push((last_x, last_y));
                        }
                    }
                    let (first_x, first_y) = points[0];
                    points.push((first_x, first_y));

                    writeln!(out_file, "{}", data_line)?;

                    // find the "visual" center of the polygon
                    province_centers.insert(province, poly_center(points));
                },

                LookState::Provinces if line.trim().starts_with("d=\"") => {
                    data_line = line.clone();
                    line = String::new();
                },

                LookState::Provinces if line.trim().starts_with("style=\"fill") => {
                    style_line = line.clone();
                    line = String::new();
                },

                LookState::Units => if line.find('>').is_some() {
                    write!(out_file, "{}", line)?;
                    line = String::new();

                    for r in self.map.iter() {
                        if let Some(ref unit) = r.unit {
                            let &(x, y) = province_centers.get(&r.province).unwrap();
                            writeln!(out_file)?;
                            match unit.unit_type {
                                UnitType::Army => write!(out_file,
r#"<g style="fill:#006400" transform="translate({},{})">
    <rect y="-0.2761367" x="-1.5930907" height="1.2319912" width="3.6534909" ry="1" rx="1" />
    <rect y="-0.9558546" x="-0.6372381" height="0.9133727" width="1.8904694" />
    <rect y="-0.7576384" x="-2.0604001" height="0.3257652" width="1.5718508" />
</g>"#,
                                    x, y)?,
                                UnitType::Fleet => write!(out_file,
r#"<g style="fill:#0000ff" transform="translate({},{})">
    <path d="M -0.58115325,-1.92628385 V 0.33837375 H 1.40624905 Z" />
    <path d="M -1.68162975,0.69317735 H 1.68162975 L 0.85722605,1.92628385 H -0.85722605 Z" />
</g>"#,
                                    x, y)?
                            }
                        }
                    }
                },

                _ => if line.trim() == "id=\"provinces\"" {
                    look_state = LookState::Provinces;
                } else if line.trim() == "id=\"units\"" {
                    look_state = LookState::Units;
                }

            }
            writeln!(out_file, "{}", line)?;
        }

        Ok(())
    }

}

fn poly_center(verts: Vec<(f32, f32)>) -> (f32, f32) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (9999999f32, 9999999f32, 0f32, 0f32);
    for p in verts.iter() {
        if p.0 < min_x { min_x = p.0; }
        if p.0 > max_x { max_x = p.0; }
        if p.1 < min_y { min_y = p.1; }
        if p.1 > max_y { max_y = p.1; }
    }

    let n = 50;
    while max_x - min_x > 2f32 && max_y - min_y > 2f32 {
        let (mut best_x, mut best_y, mut best_dist) = (0f32, 0f32, 0f32);
        let (dx, dy) = ((max_x - min_x) / (n as f32), (max_y - min_y) / (n as f32));
        for i in 1..n {
            for j in 1..n {
                let (i, j) = (i as f32, j as f32);
                let (test_x, test_y) = (min_x + i*dx, min_y + j*dy);
                let test_dist = poly_distance(&verts, (test_x, test_y));
                // you are about to experience true poetry
                // please read the following code out loud
                if test_dist > best_dist {
                    best_x = test_x;
                    best_y = test_y;
                    best_dist = test_dist;
                }
            }
        }
        min_x = best_x - dx;
        max_x = best_x + dx;
        min_y = best_y - dy;
        max_y = best_y + dy;
    }

    ((min_x + max_x) / 2f32, (min_y + max_y) / 2f32)
}

// this is one of those things where you just sort of accept it and move on
fn poly_distance(verts: &Vec<(f32, f32)>, p: (f32, f32)) -> f32 {
    let mut dist = 9999999f32;
    let mut inside = false;
    for it in verts.windows(2) {
        if ((it[0].1 > p.1) ^ (it[1].1 > p.1)) &&
            p.0 < (it[1].0 - it[0].0) * (p.1 - it[0].1) / (it[1].1 - it[0].1)
                + it[0].0 { inside = !inside; }
        let t = (((it[1].0 - it[0].0) * (p.0 - it[0].0) +
                  (it[1].1 - it[0].1) * (p.1 - it[0].1)) /
                 ((it[0].0 - it[1].0).powi(2) +
                  (it[0].1 - it[1].1).powi(2))).max(0f32).min(1f32);
        dist = dist.min(
            (it[0].0 + t * (it[1].0 - it[0].0) - p.0).powi(2) +
            (it[0].1 + t * (it[1].1 - it[0].1) - p.1).powi(2));
    }
    if inside { dist } else { 0f32 }
}
