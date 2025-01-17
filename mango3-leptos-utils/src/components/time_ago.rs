use chrono::{DateTime, SecondsFormat, Utc};
use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::i18n::{t_string, use_i18n};

#[component]
pub fn TimeAgo(value: DateTime<Utc>) -> impl IntoView {
    let i18n = use_i18n();
    let interval = RwSignal::new(1000);
    let time_delta = RwSignal::new(Utc::now() - value);
    let value_rfc3339 = value.to_rfc3339_opts(SecondsFormat::Secs, true);

    Effect::new(move || {
        use_interval_fn(
            move || {
                let dt = Utc::now() - value;

                time_delta.set(dt);

                interval.set(match dt.num_seconds() {
                    ..60 => 1000,
                    60..3600 => 60000,
                    _ => 3600000,
                });
            },
            interval,
        );
    });

    view! {
        <time title=value_rfc3339.clone() datetime=value_rfc3339>
            {move || {
                let num_seconds = time_delta.get().num_seconds();
                let num_days = time_delta.get().num_days();
                match num_seconds {
                    1 => t_string!(i18n, shared.one_second_ago).to_owned(),
                    0 | 2..=59 => t_string!(i18n, shared.count_seconds_ago, count = num_seconds),
                    60..=119 => t_string!(i18n, shared.one_minute_ago).to_owned(),
                    120..=3599 => t_string!(i18n, shared.count_minutes_ago, count = time_delta.get().num_minutes()),
                    3600..=3659 => t_string!(i18n, shared.one_hour_ago).to_owned(),
                    3660..=7199 => t_string!(i18n, shared.more_than_an_hour_ago).to_owned(),
                    7200..=86399 => t_string!(i18n, shared.count_hours_ago, count = time_delta.get().num_hours()),
                    86400..=86459 => t_string!(i18n, shared.one_day_ago).to_owned(),
                    86460..=172799 => t_string!(i18n, shared.more_than_a_day_ago).to_owned(),
                    172800..=604799 => t_string!(i18n, shared.count_days_ago, count = num_days),
                    604800..=604859 => t_string!(i18n, shared.one_week_ago).to_owned(),
                    604860..=1209599 => t_string!(i18n, shared.more_than_a_week_ago).to_owned(),
                    1209600..=2591999 => t_string!(i18n, shared.count_weeks_ago, count = time_delta.get().num_weeks()),
                    2592000..=2592059 => t_string!(i18n, shared.one_month_ago).to_owned(),
                    2592060..=5183999 => t_string!(i18n, shared.more_than_a_month_ago).to_owned(),
                    5184000..=31535999 => t_string!(i18n, shared.count_months_ago, count = num_days / 30),
                    31536000..=31536059 => t_string!(i18n, shared.one_year_ago).to_owned(),
                    31536060..=63071999 => t_string!(i18n, shared.more_than_a_year_ago).to_owned(),
                    _ => t_string!(i18n, shared.count_years_ago, count = num_days / 365),
                }
            }}
        </time>
    }
}
