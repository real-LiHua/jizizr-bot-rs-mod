use std::collections::HashMap;
use std::fs;

use crate::BotError;
use charts_rs::{
    LineChart, Series, THEME_GRAFANA, get_font_families, get_or_try_init_fonts, svg_to_webp,
};

pub fn paint(datas: HashMap<String, HashMap<u8, f32>>) -> Result<Vec<u8>, BotError> {
    let buf = fs::read("data/freq.ttf").unwrap();
    let _ = get_or_try_init_fonts(Some(vec![&buf]));

    let series = datas
        .into_iter()
        .map(|(k, v)| Series {
            name: k,
            data: (0..24).map(|i| *v.get(&i).unwrap_or(&0.0)).collect(),
            label_show: true,
            ..Default::default()
        })
        .collect();

    let mut line_chart = LineChart::new_with_theme(
        series,
        (0..24).map(|i| i.to_string()).collect(),
        THEME_GRAFANA,
    );

    for family in get_font_families().unwrap().iter() {
        let name = family.to_string();
        if name != "Roboto".to_string() {
            line_chart.font_family = name;
            break;
        }
    }
    line_chart.series_smooth = true;
    svg_to_webp(&line_chart.svg()?)
        .map_err(|e| BotError::Custom(format!("failed to convert svg to webp: {}", e)))
}
