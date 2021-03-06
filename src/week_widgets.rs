use crate::day_report::*;
use crate::engine::*;
use crate::styles::*;
use crate::widget_generators::*;

use fltk::{app, prelude::*,
    group};

use super::{WIND_SIZE, THEME};

pub fn week_lay(s: &app::Sender<Mess>) -> Box<dyn FnMut(Mess)>  {
    let mut n_week = 0;
    let mut chrono = false;
    let wind_size = WIND_SIZE.clone();
    // Layout dei widget
    let mut week_column = group::Pack::new(40,100,wind_size.0-80,700,"");
    week_column.set_spacing(35);
    to_default_style(&mut week_column, &THEME);
    week_column.end();

    let mut week_button_row = group::Pack::new(0,0,100,50,"").with_type(group::PackType::Horizontal);
    week_button_row.set_spacing((wind_size.0-80-3*100)/2);
    week_button_row.end();
    week_column.add(&week_button_row);
    week_buttons(&mut week_button_row, s);


    let mut flexx = group::Flex::new(0,0,1400-20,700,"").row();
    to_default_style(&mut flexx, &THEME);
    flexx.end();
    let scroll = wrap_in_scroll(&flexx, (0,0,1400-20,580));
    week_column.add(&scroll);

    // Inizializzazione
    week_text(n_week, &mut flexx);

    // Azioni dei messaggi
    let week_clos = move |msg| {
            match msg {
                Mess::Prev => {n_week-=1; week_text(n_week, &mut flexx)},
                Mess::Next => {n_week+=1; week_text(n_week, &mut flexx)},
                Mess::Chrono => {
                    if chrono {
                        week_text(n_week, &mut flexx);
                        chrono = false;
                    } else {
                        week_chrono(n_week, &mut flexx);
                        chrono = true;
                    }},
                _ => {}
            }
        };
    Box::new(week_clos)
}

// Bottoni superiori
fn week_buttons(parent: &mut group::Pack, sender: &app::Sender<Mess>) {
    let mut button_prev = create_button("Previous",(100,50));
    button_prev.emit(sender.clone(), Mess::Prev);
    let mut button_chrono = create_button("Chrono",(100,50));
    button_chrono.emit(sender.clone(), Mess::Chrono);
    let mut button_next = create_button("Next",(100,50));
    button_next.emit(sender.clone(), Mess::Next);
    parent.add(&button_prev);
    parent.add(&button_chrono);
    parent.add(&button_next);

}

// Update dei giorni
pub fn week_text(n_week: i64, row: &mut group::Flex) {
    row.clear();
    let week = WeekReport::new(&rec_folder().join("RecordTime"),n_week).unwrap_or(WeekReport::default());
    for i in 0..7 {
        if let Some(day)=&week.day_reports[i] {
            let txt = create_text_widget(&format!("{}",day));
            row.add(&txt);
        } else {
            let txt = create_text_widget("No data");
            row.add(&txt);
        }
    }
    let txt = create_text_widget(&format!("{}",&week.tot_report));
    row.add(&txt);
}

// Chrono
pub fn week_chrono(n_week: i64, row: &mut group::Flex) {
    row.clear();
    let inp = Vec::from_folder(&rec_folder().join("RecordTime")).unwrap();
    let last_week = retrieve_days(&inp, n_week);
    for i in 1..8 {
        if let Some(day)=&last_week[i] {
            let txt = create_text_widget(&day.display());
            row.add(&txt);
        } else {
            let txt = create_text_widget("No data");
            row.add(&txt);
        }
    }
    let txt = create_text_widget("");
    row.add(&txt);
}
