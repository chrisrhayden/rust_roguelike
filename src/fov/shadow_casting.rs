/// recursive shadowcasting
/// from this roguebasin article
/// http://www.roguebasin.com/index.php/FOV_using_recursive_shadowcasting_-_improved
///
use crate::map::map_data::ViewTile;

const MULTIPLIERS: [[i32; 4]; 8] = [
    [1, 0, 0, 1],
    [0, 1, 1, 0],
    [0, -1, 1, 0],
    [-1, 0, 0, 1],
    [-1, 0, 0, -1],
    [0, -1, -1, 0],
    [0, 1, -1, 0],
    [1, 0, 0, -1],
];

#[derive(Clone)]
struct ShadowcastData {
    width: i32,
    height: i32,
    start_x: i32,
    start_y: i32,
    radius: i32,
    row: i32,
    start_slope: f32,
    end_slope: f32,
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
}

pub fn shadow_casting(
    tile_map: &mut [ViewTile],
    width: u32,
    height: u32,
    start_x: i32,
    start_y: i32,
    radius: u32,
) {
    let s_data = ShadowcastData {
        width: width as i32,
        height: height as i32,
        start_x: start_x as i32,
        start_y: start_y as i32,
        radius: radius as i32,
        row: 1,
        start_slope: 1.0,
        end_slope: 0.0,
        xx: 0,
        xy: 0,
        yx: 0,
        yy: 0,
    };

    // shadowcast for all 8 octants
    for mult in &MULTIPLIERS {
        let mut new_data = s_data.clone();
        new_data.xx = mult[0];
        new_data.xy = mult[1];
        new_data.yx = mult[2];
        new_data.yy = mult[3];

        cast_light(tile_map, new_data);
    }
}

fn cast_light(tile_map: &mut [ViewTile], mut s_data: ShadowcastData) {
    let mut new_start: f32 = 0.0;

    if s_data.start_slope < s_data.end_slope {
        return;
    }

    let mut blocked = false;

    let mut distance = s_data.row;

    while distance <= s_data.radius && !blocked {
        let delta_y = -distance;

        for delta_x in -distance..=0 {
            let current_x: i32 =
                s_data.start_x + delta_x * s_data.xx + delta_y * s_data.xy;
            let current_y: i32 =
                s_data.start_y + delta_x * s_data.yx + delta_y * s_data.yy;
            let slope_l = (delta_x as f32 - 0.5) / (delta_y as f32 + 0.5);
            let slope_r = (delta_x as f32 + 0.5) / (delta_y as f32 - 0.5);

            if !(current_x >= 0
                && current_y >= 0
                && current_x < s_data.width
                && current_y < s_data.height)
                || s_data.start_slope < slope_r
            {
                continue;
            } else if s_data.end_slope > slope_l {
                break;
            }

            let index = (current_x + (current_y * s_data.width)) as usize;

            if delta_x < s_data.radius && delta_y < s_data.radius {
                tile_map[index].visible = true;
            }

            if blocked {
                if tile_map[index].blocked {
                    new_start = slope_r;
                    continue;
                } else {
                    blocked = false;
                    s_data.start_slope = new_start;
                }
            } else if tile_map[index].blocked && distance < s_data.radius {
                blocked = true;
                let mut new_data = s_data.clone();

                new_data.row = distance + 1;
                new_data.end_slope = slope_l;

                cast_light(tile_map, new_data);

                new_start = slope_r;
            }
        }

        distance += 1;
    }
}
