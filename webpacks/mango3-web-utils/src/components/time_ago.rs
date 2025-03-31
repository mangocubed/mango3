use chrono::{DateTime, SecondsFormat, Utc};
use leptos::either::{EitherOf16, EitherOf4};
use leptos::prelude::*;
use leptos_use::use_interval_fn;

use crate::i18n::{t, use_i18n};

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
                    1 => EitherOf16::A(t!(i18n, shared.one_second_ago)),
                    0 | 2..=59 => EitherOf16::B(t!(i18n, shared.count_seconds_ago, count = num_seconds)),
                    60..=119 => EitherOf16::C(t!(i18n, shared.one_minute_ago)),
                    120..=3599 => {
                        EitherOf16::D(t!(i18n, shared.count_minutes_ago, count = time_delta.get().num_minutes()))
                    }
                    3600..=3659 => EitherOf16::E(t!(i18n, shared.one_hour_ago)),
                    3660..=7199 => EitherOf16::F(t!(i18n, shared.more_than_an_hour_ago)),
                    7200..=86399 => {
                        EitherOf16::G(t!(i18n, shared.count_hours_ago, count = time_delta.get().num_hours()))
                    }
                    86400..=86459 => EitherOf16::H(t!(i18n, shared.one_day_ago)),
                    86460..=172799 => EitherOf16::I(t!(i18n, shared.more_than_a_day_ago)),
                    172800..=604799 => EitherOf16::J(t!(i18n, shared.count_days_ago, count = num_days)),
                    604800..=604859 => EitherOf16::K(t!(i18n, shared.one_week_ago)),
                    604860..=1209599 => EitherOf16::L(t!(i18n, shared.more_than_a_week_ago)),
                    1209600..=2591999 => {
                        EitherOf16::M(t!(i18n, shared.count_weeks_ago, count = time_delta.get().num_weeks()))
                    }
                    2592000..=2592059 => EitherOf16::N(t!(i18n, shared.one_month_ago)),
                    2592060..=5183999 => EitherOf16::O(t!(i18n, shared.more_than_a_month_ago)),
                    num_seconds => {
                        EitherOf16::P(
                            match num_seconds {
                                5184000..=31535999 => {
                                    EitherOf4::A(t!(i18n, shared.count_months_ago, count = num_days / 30))
                                }
                                31536000..=31536059 => EitherOf4::B(t!(i18n, shared.one_year_ago)),
                                31536060..=63071999 => EitherOf4::C(t!(i18n, shared.more_than_a_year_ago)),
                                _ => EitherOf4::D(t!(i18n, shared.count_years_ago, count = num_days / 365)),
                            },
                        )
                    }
                }
            }}
        </time>
    }
}
