use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::Time,
    chrono::{DateTime, Duration, Utc},
};

use kube::{Resource, ResourceExt};

pub fn remove_managed_fields<K: Resource>(mut obj: K) -> K {
    obj.managed_fields_mut().clear();
    obj
}

pub static UNKNOWN: &str = "Unknown";
pub static NOT_AVAILABLE: &str = "N/A";
pub static NONE: &str = "None";

pub fn to_age(timestamp: Option<&Time>, against: DateTime<Utc>) -> String {
    match timestamp {
        Some(time) => {
            let time = time.0;
            let duration = against.signed_duration_since(time);

            duration_to_age(duration, false)
        }
        None => String::default(),
    }
}

pub fn to_age_secs(timestamp: Option<&Time>, against: DateTime<Utc>) -> String {
    match timestamp {
        Some(time) => {
            let time = time.0;
            let duration = against.signed_duration_since(time);

            duration_to_age(duration, true)
        }
        None => String::default(),
    }
}

pub fn duration_to_age(duration: Duration, with_secs: bool) -> String {
    let mut out = String::new();

    let weeks = duration.num_weeks();
    let mut remaining_duration = duration - Duration::weeks(weeks);

    if weeks != 0 {
        out.push_str(&format!("{}w", weeks));
    }

    let days: i64 = remaining_duration.num_days();
    remaining_duration = remaining_duration - Duration::days(days);

    if days != 0 {
        out.push_str(&format!("{}d", days));
    }

    let hrs = remaining_duration.num_hours();
    remaining_duration = remaining_duration - Duration::hours(hrs);

    if hrs != 0 {
        out.push_str(&format!("{}h", hrs));
    }

    let mins = remaining_duration.num_minutes();

    if mins != 0 && days == 0 && weeks == 0 {
        out.push_str(&format!("{}m", mins));
    }

    if with_secs {
        let secs = remaining_duration.num_seconds() - (mins * 60);
        if secs != 0 && hrs == 0 && days == 0 && weeks == 0 {
            out.push_str(&format!("{}s", secs));
        }
    }

    if out.is_empty() && with_secs {
        "0s".into()
    } else if out.is_empty() {
        "0m".into()
    } else {
        out
    }
}
